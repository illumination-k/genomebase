//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.0

use super::sea_orm_active_enums::Category;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "kogs")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub category: Category,
    #[sea_orm(column_type = "Text")]
    pub description: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl Related<super::transcripts::Entity> for Entity {
    fn to() -> RelationDef {
        super::kog_annotations::Relation::Transcripts.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::kog_annotations::Relation::Kogs.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
