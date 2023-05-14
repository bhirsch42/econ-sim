use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "production_strategy")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub duration: i32,
    pub role: super::agent::Role,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::production_requirement::Entity")]
    ProductionRequirement,
}

impl Related<super::production_requirement::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ProductionRequirement.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
