use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "commodity")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::trade_offer::Entity")]
    TradeOffer,
}

impl Related<super::trade_offer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TradeOffer.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
