use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(Organism::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Organism::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Organism::Name).string().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(GenomeVersion::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(GenomeVersion::Version).string().not_null())
                    .col(
                        ColumnDef::new(GenomeVersion::TaxonomyId)
                            .string()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .col(GenomeVersion::Version)
                            .col(GenomeVersion::TaxonomyId)
                            .primary(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_organism_to_genomeversion")
                            .from(GenomeSequence::Table, GenomeVersion::TaxonomyId)
                            .to(Organism::Table, Organism::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(GenomeSequence::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(GenomeSequence::TaxonomyId)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(GenomeSequence::Version).string().not_null())
                    .col(ColumnDef::new(GenomeSequence::SeqName).string().not_null())
                    .col(ColumnDef::new(GenomeSequence::Sequence).text().not_null())
                    .primary_key(
                        Index::create()
                            .col(GenomeSequence::TaxonomyId)
                            .col(GenomeSequence::Version)
                            .col(GenomeSequence::SeqName)
                            .primary(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_organism_to_genomesequence")
                            .from(GenomeSequence::Table, GenomeSequence::TaxonomyId)
                            .to(Organism::Table, Organism::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_genomeversion_to_genomesequence")
                            .from_tbl(GenomeSequence::Table)
                            .from_col(GenomeSequence::TaxonomyId)
                            .from_col(GenomeSequence::Version)
                            .to_tbl(GenomeVersion::Table)
                            .to_col(GenomeVersion::Version)
                            .to_col(GenomeVersion::TaxonomyId)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(AnnotationModelVersion::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AnnotationModelVersion::TaxonomyId)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AnnotationModelVersion::Version)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AnnotationModelVersion::GenomeVersion)
                            .string()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .col(AnnotationModelVersion::TaxonomyId)
                            .col(AnnotationModelVersion::Version)
                            .primary(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from_tbl(AnnotationModelVersion::Table)
                            .from_col(AnnotationModelVersion::TaxonomyId)
                            .from_col(AnnotationModelVersion::GenomeVersion)
                            .to_tbl(GenomeVersion::Table)
                            .to_col(GenomeVersion::TaxonomyId)
                            .to_col(GenomeVersion::Version)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(AnnotationModelVersion::Table, AnnotationModelVersion::TaxonomyId)
                            .to(Organism::Table, Organism::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(
                Table::drop()
                    .table(AnnotationModelVersion::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(GenomeSequence::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(GenomeVersion::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Organism::Table).to_owned())
            .await?;

        Ok(())
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Organism {
    Table,
    #[iden = "taxonomy_id"]
    Id,
    Name,
}

#[derive(Iden)]
enum GenomeVersion {
    Table,
    Version,
    TaxonomyId,
}

#[derive(Iden)]
enum GenomeSequence {
    Table,
    Version,
    TaxonomyId,
    SeqName,
    Sequence,
}

#[derive(Iden)]
enum AnnotationModelVersion {
    Table,
    TaxonomyId,
    Version,
    GenomeVersion,
}
