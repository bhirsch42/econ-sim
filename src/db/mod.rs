use crate::models;
use sea_orm::entity::prelude::*;
use sea_orm::Set;
use sea_orm::{sea_query::TableCreateStatement, DbBackend, DbConn, DbErr, Schema};

const AGENTS_COUNT: usize = 5;

pub async fn seed(db: &DbConn) -> Result<(), DbErr> {
    // Agents
    let agents = (0..AGENTS_COUNT).map(|_| models::agent::ActiveModel {
        role: Set(rand::random()),
        ..Default::default()
    });

    models::agent::Entity::insert_many(agents).exec(db).await?;

    // Commodity: Food
    let commodity_food = models::commodity::ActiveModel {
        name: Set("Food".to_owned()),
        ..Default::default()
    };

    let commodity_id_food = commodity_food.insert(db).await?.id;

    // Commodity: Water
    let commodity_water = models::commodity::ActiveModel {
        name: Set("Water".to_owned()),
        ..Default::default()
    };

    let commodity_id_water = commodity_water.insert(db).await?.id;

    // Insert agents and commodities
    let find_agents = models::agent::Entity::find().all(db);
    let find_commodities = models::commodity::Entity::find().all(db);
    let (agents, commodities) = futures::try_join!(find_agents, find_commodities)?;

    // Inventories
    let inventories = agents.iter().flat_map(|agent| {
        return commodities
            .iter()
            .map(|commodity| models::inventory::ActiveModel {
                commodity_id: Set(commodity.id),
                agent_id: Set(agent.id),
                amount: Set(10),
                ideal_amount: Set(50),
                max_amount: Set(100),
                ..Default::default()
            });
    });

    let insert_inventories = models::inventory::Entity::insert_many(inventories).exec(db);
    insert_inventories.await?;

    // Production strategy: Farmer
    let production_strategy_farmer = models::production_strategy::ActiveModel {
        role: Set(models::agent::Role::Farmer),
        duration: Set(1),
        ..Default::default()
    };

    let production_strategy_id_farmer = production_strategy_farmer.insert(db).await?.id;

    // Production strategy: Water Source
    let production_strategy_water_source = models::production_strategy::ActiveModel {
        role: Set(models::agent::Role::WaterSource),
        duration: Set(1),
        ..Default::default()
    };

    let production_strategy_id_water_source = production_strategy_water_source.insert(db).await?.id;

    let production_requirements = [
        // Produce Food
        models::production_requirement::ActiveModel {
            production_strategy_id: Set(production_strategy_id_farmer),
            commodity_id: Set(commodity_id_water),
            production_requirement_type: Set(
                models::production_requirement::ProductionRequirementType::Input,
            ),
            amount: Set(1),
            ..Default::default()
        },
        models::production_requirement::ActiveModel {
            production_strategy_id: Set(production_strategy_id_farmer),
            commodity_id: Set(commodity_id_food),
            production_requirement_type: Set(
                models::production_requirement::ProductionRequirementType::Output,
            ),
            amount: Set(1),
            ..Default::default()
        },
        // Produce Water
        models::production_requirement::ActiveModel {
            production_strategy_id: Set(production_strategy_id_water_source),
            commodity_id: Set(commodity_id_water),
            production_requirement_type: Set(
                models::production_requirement::ProductionRequirementType::Output,
            ),
            amount: Set(1),
            ..Default::default()
        },
    ];

    models::production_requirement::Entity::insert_many(production_requirements)
        .exec(db)
        .await?;

    Ok(())
}

async fn create_agent_table(db: &DbConn) -> Result<(), DbErr> {
    let schema = Schema::new(DbBackend::Sqlite);
    let stmt: TableCreateStatement = schema.create_table_from_entity(models::agent::Entity);
    db.execute(db.get_database_backend().build(&stmt)).await?;
    Ok(())
}

async fn create_commodity_table(db: &DbConn) -> Result<(), DbErr> {
    let schema = Schema::new(DbBackend::Sqlite);
    let stmt: TableCreateStatement = schema.create_table_from_entity(models::commodity::Entity);
    db.execute(db.get_database_backend().build(&stmt)).await?;
    Ok(())
}

async fn create_inventory_table(db: &DbConn) -> Result<(), DbErr> {
    let schema = Schema::new(DbBackend::Sqlite);
    let stmt: TableCreateStatement = schema.create_table_from_entity(models::inventory::Entity);
    db.execute(db.get_database_backend().build(&stmt)).await?;
    Ok(())
}

async fn create_production_strategy_table(db: &DbConn) -> Result<(), DbErr> {
    let schema = Schema::new(DbBackend::Sqlite);
    let stmt: TableCreateStatement =
        schema.create_table_from_entity(models::production_strategy::Entity);
    db.execute(db.get_database_backend().build(&stmt)).await?;
    Ok(())
}

async fn create_production_requirement_table(db: &DbConn) -> Result<(), DbErr> {
    let schema = Schema::new(DbBackend::Sqlite);
    let stmt: TableCreateStatement =
        schema.create_table_from_entity(models::production_requirement::Entity);
    db.execute(db.get_database_backend().build(&stmt)).await?;
    Ok(())
}

pub async fn setup(db: &DbConn) -> Result<(), DbErr> {
    futures::try_join!(
        create_agent_table(db),
        create_commodity_table(db),
        create_inventory_table(db),
        create_production_strategy_table(db),
        create_production_requirement_table(db),
    )?;
    Ok(())
}

pub async fn read_debug(db: &DbConn) -> Result<(), DbErr> {
    let agents = models::agent::Entity::find()
        .find_with_related(models::inventory::Entity)
        .all(db);

    let commodities = models::commodity::Entity::find().all(db);

    let results = futures::try_join!(agents, commodities)?;

    println!("{results:#?}");

    Ok(())
}
