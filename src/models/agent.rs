use rand::{distributions::Standard, prelude::*};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "agent")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub role: Role,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::trade_offer::Entity")]
    TradeOffer,
    #[sea_orm(has_many = "super::inventory::Entity")]
    Inventory,
}

impl Related<super::trade_offer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TradeOffer.def()
    }
}

impl Related<super::inventory::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Inventory.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, Hash, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(Some(1))")]
pub enum Role {
    #[sea_orm(string_value = "farmer")]
    Farmer,
    #[sea_orm(string_value = "water_source")]
    WaterSource,
}

impl Distribution<Role> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Role {
        match rng.gen_range(0..2) {
            0 => Role::Farmer,
            _ => Role::WaterSource,
        }
    }
}
