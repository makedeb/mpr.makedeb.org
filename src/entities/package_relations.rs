//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "PackageRelations")]
pub struct Model {
    /// SeaORM requires all table to have a primary key, so add one here. See https://github.com/SeaQL/sea-orm/issues/485>.
    #[sea_orm(column_name = "ID", primary_key)]
    pub id: u32,
    #[sea_orm(column_name = "PackageID")]
    pub package_id: u32,
    #[sea_orm(column_name = "RelTypeID")]
    pub rel_type_id: u8,
    #[sea_orm(column_name = "RelName")]
    pub rel_name: String,
    #[sea_orm(column_name = "RelCondition")]
    pub rel_condition: Option<String>,
    #[sea_orm(column_name = "RelArch")]
    pub rel_arch: Option<String>,
    #[sea_orm(column_name = "RelDist")]
    pub rel_dist: Option<String>,
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
        belongs_to = "super::relation_types::Entity",
        from = "Column::RelTypeId",
        to = "super::relation_types::Column::Id",
        on_update = "Restrict",
        on_delete = "NoAction"
    )]
    RelationTypes,
}

impl Related<super::packages::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Packages.def()
    }
}

impl Related<super::relation_types::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RelationTypes.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
