//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "TU_VoteInfo")]
pub struct Model {
    #[sea_orm(column_name = "ID", primary_key)]
    pub id: u32,
    #[sea_orm(column_name = "Agenda", column_type = "Text")]
    pub agenda: String,
    #[sea_orm(column_name = "User")]
    pub user: String,
    #[sea_orm(column_name = "Submitted")]
    pub submitted: u64,
    #[sea_orm(column_name = "End")]
    pub end: u64,
    #[sea_orm(column_name = "Quorum", column_type = "Decimal(Some((2, 2)))")]
    pub quorum: Decimal,
    #[sea_orm(column_name = "SubmitterID")]
    pub submitter_id: u32,
    #[sea_orm(column_name = "Yes")]
    pub yes: u32,
    #[sea_orm(column_name = "No")]
    pub no: u32,
    #[sea_orm(column_name = "Abstain")]
    pub abstain: u32,
    #[sea_orm(column_name = "ActiveTUs")]
    pub active_t_us: u32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::SubmitterId",
        to = "super::users::Column::Id",
        on_update = "Restrict",
        on_delete = "Cascade"
    )]
    Users,
    #[sea_orm(has_many = "super::tu_votes::Entity")]
    TuVotes,
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}

impl Related<super::tu_votes::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TuVotes.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
