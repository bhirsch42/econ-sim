use db::read_debug;
use production::{get_production_strategies, perform_production};
use sea_orm::{Database, DbErr};
mod db;
mod models;

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    let db = Database::connect("sqlite::memory:").await?;

    db::setup(&db).await?;
    db::seed(&db).await?;

    let production_strategy_map = get_production_strategies(&db).await?;

    perform_production(&db, &production_strategy_map).await?;
    perform_production(&db, &production_strategy_map).await?;
    perform_production(&db, &production_strategy_map).await?;

    read_debug(&db).await?;

    Ok(())
}

mod production;
