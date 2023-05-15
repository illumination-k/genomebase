use std::collections::HashMap;

use anyhow::{anyhow, Result};

use genome_domain::{AnnotationVersion, GenomeVersion, IOrganismRepository, Organism};
use sqlx::{mysql::MySqlPool, Execute, MySql, QueryBuilder};
use uuid::{Error, Uuid};

pub struct OrganismRepository {
    pool: MySqlPool,
}

impl OrganismRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl IOrganismRepository for OrganismRepository {
    async fn get_organism(&self, taxonomy_id: u32) -> Result<Organism> {
        let records = sqlx::query!(
            r#"
            SELECT
                o.taxonomy_id as taxonomy_id,
                o.name as name,
                gv.id as genome_version_id,
                gv.major_version as genome_version_major_version,
                gv.minor_version as genome_version_minor_version,
                gv.patch_version as genome_version_patch_version,
                av.id as annotation_version_id,
                av.major_version as annotation_version_major_version,
                av.minor_version as annotation_version_minor_version,
                av.patch_version as annotation_version_patch_version
            FROM
                organisms o
            LEFT JOIN
                genome_versions gv ON gv.taxonomy_id = o.taxonomy_id
            LEFT JOIN
                annotation_versions av ON av.genome_version_id = gv.id
            WHERE
                o.taxonomy_id = ?
            "#,
            taxonomy_id
        )
        .fetch_all(&self.pool)
        .await?;

        let mut organism = None;
        let mut cur_genome_version_id = None;

        let mut genome_version_to_annotation_versions = HashMap::new();
        for record in records.into_iter() {
            if organism.is_none() {
                organism = Some(Organism::new(record.taxonomy_id, record.name, vec![]))
            }

            if cur_genome_version_id.is_none() || cur_genome_version_id != record.genome_version_id
            {
                cur_genome_version_id = record.genome_version_id.clone();
            }

            if let (
                Some(genome_version_id),
                Some(genome_major_version),
                Some(genome_minor_version),
                Some(genome_patch_version),
            ) = (
                cur_genome_version_id.clone(),
                record.genome_version_major_version,
                record.genome_version_minor_version,
                record.genome_version_patch_version,
            ) {
                organism.as_mut().map(|o| -> Result<(), Error> {
                    o.push_genome_version(GenomeVersion::new(
                        Uuid::from_slice(&genome_version_id)?,
                        genome_major_version as u8,
                        genome_minor_version as u8,
                        genome_patch_version as u8,
                        vec![],
                    ));
                    Ok(())
                });

                if let (
                    Some(annotation_version_id),
                    Some(annotation_version_major),
                    Some(annotation_version_minor),
                    Some(annotation_version_patch),
                ) = (
                    record.annotation_version_id,
                    record.annotation_version_major_version,
                    record.annotation_version_minor_version,
                    record.annotation_version_patch_version,
                ) {
                    genome_version_to_annotation_versions
                        .entry(genome_version_id)
                        .or_insert_with(|| vec![])
                        .push(AnnotationVersion::new(
                            Uuid::from_slice(&annotation_version_id)?,
                            annotation_version_major as u8,
                            annotation_version_minor as u8,
                            annotation_version_patch as u8,
                        ));
                }
            }
        }

        organism.as_mut().map(|o| {
            o.genome_versions_mut().iter_mut().for_each(|gv| {
                if let Some(annotation_versions) =
                    genome_version_to_annotation_versions.get(gv.id().as_bytes().as_slice())
                {
                    gv.extend_annotation_versions(annotation_versions.clone());
                }
            })
        });

        organism.ok_or(anyhow!("Organism not found"))
    }

    async fn list_organisms(&self) -> Result<Vec<Organism>> {
        let records = sqlx::query!(
            r#"
            SELECT
                o.taxonomy_id,
                o.name,
                g.id as genome_version_id,
                g.major_version as genome_major,
                g.minor_version as genome_minor,
                g.patch_version as genome_patch,
                a.major_version as annotation_major,
                a.minor_version as annotation_minor,
                a.patch_version as annotation_patch
            FROM
                organisms o
            LEFT JOIN
                genome_versions g ON o.taxonomy_id = g.taxonomy_id
            LEFT JOIN
                annotation_versions a ON g.id = a.genome_version_id
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        let mut organism_map: HashMap<u32, Organism> = HashMap::new();

        Ok(organism_map.into_values().collect())
    }

    async fn save_organism(&self, organism: &Organism) -> Result<()> {
        let mut transaction = self.pool.begin().await?;
        sqlx::query!(
            r#"
            INSERT INTO
                organisms (taxonomy_id, name)
            VALUES
                (?, ?)
            ON DUPLICATE KEY UPDATE
                taxonomy_id = ?
            "#,
            organism.taxonomy_id(),
            organism.name(),
            organism.taxonomy_id()
        )
        .execute(&mut transaction)
        .await?;

        let mut annotation_version_query_builder = QueryBuilder::<MySql>::new(
            "INSERT IGNORE INTO annotation_versions (id, genome_version_id, major_version, minor_version, patch_version) ",
        );

        let mut genome_version_query_builder = QueryBuilder::<MySql>::new(
            r#"INSERT INTO
                genome_versions (id, taxonomy_id, major_version, minor_version, patch_version) 
            "#,
        );

        genome_version_query_builder.push_values(
            organism.genome_versions().iter(),
            |mut b, genome_version| {
                b.push_bind(genome_version.id().as_bytes().as_slice())
                    .push_bind(organism.taxonomy_id())
                    .push_bind(genome_version.major())
                    .push_bind(genome_version.minor())
                    .push_bind(genome_version.patch());

                annotation_version_query_builder.push_values(
                    genome_version.annotation_versions().iter(),
                    |mut b, annotation_version| {
                        b.push_bind(annotation_version.id().as_bytes().to_vec())
                            .push_bind(genome_version.id().as_bytes().as_slice())
                            .push_bind(annotation_version.major())
                            .push_bind(annotation_version.minor())
                            .push_bind(annotation_version.patch());
                    },
                );
            },
        );

        genome_version_query_builder.push(
            r#"
            ON DUPLICATE KEY UPDATE
                major_version = VALUES(major_version),
                minor_version = VALUES(minor_version),
                patch_version = VALUES(patch_version)
            "#,
        );

        dbg!(
            genome_version_query_builder
                .build()
                .execute(&mut transaction)
                .await?
        );

        annotation_version_query_builder
            .build()
            .execute(&mut transaction)
            .await?;

        transaction.commit().await?;
        Ok(())
    }

    async fn delete_organism(&self, taxonomy_id: u32) -> Result<()> {
        let mut transaction = self.pool.begin().await?;
        sqlx::query!(
            r#"
            DELETE FROM
                organisms
            WHERE
                taxonomy_id = ?
            "#,
            taxonomy_id
        )
        .execute(&mut transaction)
        .await?;

        transaction.commit().await?;
        Ok(())
    }
}
