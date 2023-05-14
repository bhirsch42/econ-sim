use crate::models;
use futures::future::try_join_all;
use sea_orm::{prelude::*, Set};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct ProductionStrategy {
    pub role: models::agent::Role,
    pub duration: i32,
    pub inputs: Vec<ProductionRequirement>,
    pub outputs: Vec<ProductionRequirement>,
}

#[derive(Clone, Copy, Debug)]
pub struct ProductionRequirement {
    pub commodity_id: i32,
    pub amount: i32,
}

impl From<&models::production_requirement::Model> for ProductionRequirement {
    fn from(value: &models::production_requirement::Model) -> Self {
        ProductionRequirement {
            commodity_id: value.commodity_id,
            amount: value.amount,
        }
    }
}

impl
    From<&(
        models::production_strategy::Model,
        Vec<models::production_requirement::Model>,
    )> for ProductionStrategy
{
    fn from(
        val: &(
            models::production_strategy::Model,
            Vec<models::production_requirement::Model>,
        ),
    ) -> Self {
        let (production_strategy, production_requirements) = val;
        let inputs: Vec<ProductionRequirement> = production_requirements
            .iter()
            .filter(|production_requirement| {
                production_requirement.production_requirement_type
                    == models::production_requirement::ProductionRequirementType::Input
            })
            .map(Into::<ProductionRequirement>::into)
            .collect();

        let outputs: Vec<ProductionRequirement> = production_requirements
            .iter()
            .filter(|production_requirement| {
                production_requirement.production_requirement_type
                    == models::production_requirement::ProductionRequirementType::Output
            })
            .map(Into::<ProductionRequirement>::into)
            .collect();

        ProductionStrategy {
            role: production_strategy.role,
            duration: 10,
            inputs,
            outputs,
        }
    }
}
type ProductionStrategyMap = HashMap<models::agent::Role, ProductionStrategy>;

pub async fn get_production_strategies(db: &DbConn) -> Result<ProductionStrategyMap, DbErr> {
    let production_strategies = models::production_strategy::Entity::find()
        .find_with_related(models::production_requirement::Entity)
        .all(db)
        .await?;

    Ok(production_strategies
        .into_iter()
        .map(|(production_strategy, production_requirements)| {
            (
                production_strategy.role,
                Into::<ProductionStrategy>::into(&(production_strategy, production_requirements)),
            )
        })
        .collect())
}

fn applied_production(
    production_strategy: &ProductionStrategy,
    inventories: &[models::inventory::Model],
) -> Vec<models::inventory::ActiveModel> {
    let inputs = &production_strategy.inputs;
    let are_inputs_satisfied = inputs.iter().all(|production_requirement| {
        let inventory = inventories
            .iter()
            .find(|inventory| inventory.commodity_id == production_requirement.commodity_id)
            .unwrap();

        inventory.amount >= production_requirement.amount
    });

    if !are_inputs_satisfied {
        return vec![];
    }

    let outputs = &production_strategy.outputs;
    let are_outputs_satisfied = outputs.iter().all(|production_requirement| {
        let inventory = inventories
            .iter()
            .find(|inventory| inventory.commodity_id == production_requirement.commodity_id)
            .unwrap();

        inventory.max_amount - inventory.amount >= production_requirement.amount
    });

    if !are_outputs_satisfied {
        return vec![];
    }

    let updated_inputs: Vec<models::inventory::ActiveModel> = inputs
        .iter()
        .map(|production_requirement| {
            let inventory: models::inventory::Model = inventories
                .iter()
                .find(|inventory| inventory.commodity_id == production_requirement.commodity_id)
                .unwrap()
                .clone();

            let amount = inventory.amount;
            let mut inventory_model: models::inventory::ActiveModel = inventory.into();

            inventory_model.amount = Set(amount - production_requirement.amount);
            inventory_model
        })
        .collect();

    let updated_outputs: Vec<models::inventory::ActiveModel> = outputs
        .iter()
        .map(|production_requirement| {
            let inventory: models::inventory::Model = inventories
                .iter()
                .find(|inventory| inventory.commodity_id == production_requirement.commodity_id)
                .unwrap()
                .clone();

            let amount = inventory.amount;
            let mut inventory_model: models::inventory::ActiveModel = inventory.into();

            inventory_model.amount = Set(amount + production_requirement.amount);
            inventory_model
        })
        .collect();

    let mut updated_inventories = updated_inputs;
    updated_inventories.extend(updated_outputs);
    updated_inventories
}

pub async fn perform_production(
    db: &DbConn,
    production_strategy_map: &ProductionStrategyMap,
) -> Result<(), DbErr> {
    let agents = models::agent::Entity::find()
        .find_with_related(models::inventory::Entity)
        .all(db)
        .await?;

    let updated_inventories: Vec<models::inventory::ActiveModel> = agents
        .into_iter()
        .flat_map(|(agent, inventories)| {
            let production_strategy = production_strategy_map.get(&agent.role).unwrap();
            applied_production(production_strategy, &inventories)
        })
        .collect();

    if updated_inventories.is_empty() {
        return Ok(());
    }

    let inventory_updates = updated_inventories
        .into_iter()
        .map(|updated_inventory| updated_inventory.save(db));

    try_join_all(inventory_updates).await?;

    Ok(())
}
