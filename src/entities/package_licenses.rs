//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "PackageLicenses")]
pub struct Model {
    #[sea_orm(column_name = "PackageID", primary_key, auto_increment = false)]
    pub package_id: u32,
    #[sea_orm(column_name = "LicenseID", primary_key, auto_increment = false)]
    pub license_id: u32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::packages::Entity",
        from = "Column::PackageId",
        to = "super::packages::Column::Id",
        on_update = "Restrict",
        on_delete = "Cascade"
    )]
    Packages,
    #[sea_orm(
        belongs_to = "super::licenses::Entity",
        from = "Column::LicenseId",
        to = "super::licenses::Column::Id",
        on_update = "Restrict",
        on_delete = "Cascade"
    )]
    Licenses,
}

impl Related<super::packages::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Packages.def()
    }
}

impl Related<super::licenses::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Licenses.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}