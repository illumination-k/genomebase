//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "kegg_reaction")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub name: String,
    #[sea_orm(column_type = "Text")]
    pub description: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl Related<super::transcripts::Entity> for Entity {
    fn to() -> RelationDef {
        super::kegg_reaction_annotation::Relation::Transcripts.def()
    }
    fn via() -> Option<RelationDef> {
        Some(
            super::kegg_reaction_annotation::Relation::KeggReaction
                .def()
                .rev(),
        )
    }
}

impl ActiveModelBehavior for ActiveModel {}
