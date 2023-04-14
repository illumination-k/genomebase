use std::collections::HashMap;

use anyhow::Result;

use genome_domain::{IOrganismRepository, Organism};
use sqlx::{mysql::MySqlPool, MySql, QueryBuilder};

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
        let record = sqlx::query!(
            r#"
            SELECT
                taxonomy_id,
                name
            FROM
                organisms
            WHERE
                taxonomy_id = ?
            "#,
            taxonomy_id
        )
        .fetch_one(&self.pool)
        .await?;

        let mut organism = Organism::new(record.taxonomy_id, record.name, vec![]);

        let genome_versions = sqlx::query!(
            r#"
            SELECT
                id,
                major_version,
                minor_version,
                patch_version
            FROM
                genome_versions
            WHERE
                taxonomy_id = ?
            "#,
            taxonomy_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(organism)
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
            "INSERT IGNORE INTO annotation_versions (genome_version_id, major_version, minor_version, patch_version) ",
        );

        let mut genome_version_query_builder = QueryBuilder::<MySql>::new(
            r#"
            INSERT INTO
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
                        b.push_bind(genome_version.id().as_bytes().as_slice())
                            .push_bind(annotation_version.major())
                            .push_bind(annotation_version.minor())
                            .push_bind(annotation_version.patch());
                    },
                );
            },
        );

        genome_version_query_builder.push(
            r#"
            ON DUPLICATE KEY UPDATE id = VALUES(id) 
            "#,
        );

        genome_version_query_builder
            .build()
            .execute(&mut transaction)
            .await?;

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
