//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "DependencyTypes")]
pub struct Model {
    #[sea_orm(column_name = "ID", primary_key)]
    pub id: u8,
    #[sea_orm(column_name = "Name")]
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::package_depends::Entity")]
    PackageDepends,
}

impl Related<super::package_depends::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PackageDepends.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
