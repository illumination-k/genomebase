//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "gene_models")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Vec<u8>,
    pub taxonomy_id: u32,
    pub genome_id: Vec<u8>,
    pub annotation_model_id: Vec<u8>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::annotation_models::Entity",
        from = "Column::AnnotationModelId",
        to = "super::annotation_models::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    AnnotationModels,
    #[sea_orm(has_many = "super::genes::Entity")]
    Genes,
    #[sea_orm(
        belongs_to = "super::genomes::Entity",
        from = "Column::GenomeId",
        to = "super::genomes::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Genomes,
    #[sea_orm(has_many = "super::gff_records::Entity")]
    GffRecords,
    #[sea_orm(
        belongs_to = "super::organisms::Entity",
        from = "Column::TaxonomyId",
        to = "super::organisms::Column::TaxonomyId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Organisms,
    #[sea_orm(has_many = "super::transcripts::Entity")]
    Transcripts,
}

impl Related<super::annotation_models::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AnnotationModels.def()
    }
}

impl Related<super::genes::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Genes.def()
    }
}

impl Related<super::genomes::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Genomes.def()
    }
}

impl Related<super::gff_records::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::GffRecords.def()
    }
}

impl Related<super::organisms::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Organisms.def()
    }
}

impl Related<super::transcripts::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Transcripts.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
