use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "inventory")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub commodity_id: i32,
    pub agent_id: i32,
    pub amount: i32,
    pub max_amount: i32,
    pub ideal_amount: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::agent::Entity",
        from = "Column::AgentId",
        to = "super::agent::Column::Id"
    )]
    Agent,
    #[sea_orm(
        belongs_to = "super::commodity::Entity",
        from = "Column::CommodityId",
        to = "super::commodity::Column::Id"
    )]
    Commodity,
}

impl Related<super::agent::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Agent.def()
    }
}

impl Related<super::commodity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Commodity.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
