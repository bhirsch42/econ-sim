use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "production_requirement")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub production_strategy_id: i32,
    pub commodity_id: i32,
    pub amount: i32,
    pub production_requirement_type: ProductionRequirementType,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::production_strategy::Entity",
        from = "Column::ProductionStrategyId",
        to = "super::production_strategy::Column::Id"
    )]
    ProductionStrategy,

    #[sea_orm(
        belongs_to = "super::commodity::Entity",
        from = "Column::CommodityId",
        to = "super::commodity::Column::Id"
    )]
    Commodity,
}

impl Related<super::production_strategy::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ProductionStrategy.def()
    }
}

impl Related<super::commodity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Commodity.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(Some(1))")]
pub enum ProductionRequirementType {
    #[sea_orm(string_value = "input")]
    Input,
    #[sea_orm(string_value = "output")]
    Output,
}
