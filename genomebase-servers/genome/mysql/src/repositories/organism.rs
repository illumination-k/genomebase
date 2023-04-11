use anyhow::Result;

use sqlx::mysql::MySqlPool;
use genome_domain::{IOrganismRepository, Organism};

pub struct OrganismRepository {
    pool: MySqlPool
}

impl OrganismRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl IOrganismRepository for OrganismRepository {
    async fn get_organism(&self, taxonomy_id: u32) -> Result<Option<Organism>> {
        let organism = sqlx::query_as!(
            Organism,
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

        Ok(organism)
    }

    async fn list_organisms(&self) -> Result<Vec<Organism>> {
        let organisms = sqlx::query_as!(
            Organism,
            r#"
            SELECT
                taxonomy_id,
                name
            FROM
                organisms
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(organisms)
    }

    async fn save_organism(&self, organism: &Organism) -> Result<()> {
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
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete_organism(&self, taxonomy_id: u32) -> Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM
                organisms
            WHERE
                taxonomy_id = ?
            "#,
            taxonomy_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}