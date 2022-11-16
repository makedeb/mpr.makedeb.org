//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "AcceptedTerms")]
pub struct Model {
    /// SeaORM requires all table to have a primary key, so add one here. See https://github.com/SeaQL/sea-orm/issues/485>.
    #[sea_orm(column_name = "ID", primary_key)]
    pub id: u32,
    #[sea_orm(column_name = "UsersID")]
    pub users_id: u32,
    #[sea_orm(column_name = "TermsID")]
    pub terms_id: u32,
    #[sea_orm(column_name = "Revision")]
    pub revision: u32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::UsersId",
        to = "super::users::Column::Id",
        on_update = "Restrict",
        on_delete = "Cascade"
    )]
    Users,
    #[sea_orm(
        belongs_to = "super::terms::Entity",
        from = "Column::TermsId",
        to = "super::terms::Column::Id",
        on_update = "Restrict",
        on_delete = "Cascade"
    )]
    Terms,
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}

impl Related<super::terms::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Terms.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}