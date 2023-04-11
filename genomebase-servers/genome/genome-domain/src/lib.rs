pub mod gene;
mod multistats_correction;

use std::collections::HashMap;

use anyhow::Result;
use derive_new::new;

use fishers_exact::fishers_exact;
use gene::IGeneRepository;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub trait TermID {
    fn try_new(id: &str) -> Result<Self>
    where
        Self: Sized;
    fn id(&self) -> &String;
}

pub fn term_id_serializer<S: Serializer, T: TermID>(term: &T, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_str(term.id())
}

pub fn term_id_deserializer<'de, D: Deserializer<'de>, T: TermID>(d: D) -> Result<T, D::Error> {
    let id = String::deserialize(d)?;
    T::try_new(&id).map_err(serde::de::Error::custom)
}

#[macro_export]
macro_rules! impl_term_serde {
    ($term_name: ident) => {
        impl ToString for $term_name {
            fn to_string(&self) -> String {
                self.id().to_owned()
            }
        }

        impl Serialize for $term_name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                term_id_serializer(self, serializer)
            }
        }

        impl<'de> Deserialize<'de> for $term_name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                term_id_deserializer(deserializer)
            }
        }
    };
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, new)]
pub struct Organism {
    taxonomy_id: u32,
    name: String,
    genome_versions: Vec<GenomeVersion>,
}

impl Organism {
    pub fn taxonomy_id(&self) -> u32 {
        self.taxonomy_id
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn genome_versions(&self) -> &Vec<GenomeVersion> {
        &self.genome_versions
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, new)]
pub struct GenomeVersion {
    major: u8,
    minor: u8,
    patch: u8,
    annotation_versions: Vec<AnnotationVersion>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, new)]

pub struct AnnotationVersion {
    major: u8,
    minor: u8,
    patch: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, new)]

pub struct GeneModel {
    taxonomy_id: u32,
    genome_version: String,
    annotation_model_version: String,
}

impl GeneModel {
    pub fn taxonomy_id(&self) -> u32 {
        self.taxonomy_id
    }

    pub fn genome_version(&self) -> &String {
        &self.genome_version
    }

    pub fn annotation_model_version(&self) -> &String {
        &self.annotation_model_version
    }
}

/*
必要な機能
- Organismの登録、取得、削除
*/
#[async_trait::async_trait]
pub trait IOrganismRepository {
    async fn save_organism(&self, organism: &Organism) -> Result<()>;
    async fn list_organisms(&self) -> Result<Vec<Organism>>;
    async fn get_organism(&self, taxonomy_id: u32) -> Result<Option<Organism>>;
    async fn delete_organism(&self, taxonomy_id: u32) -> Result<()>;
}

pub struct GenomeService {
    pub organism_repository: Box<dyn IOrganismRepository>,
    gene_repositories: HashMap<GeneModel, Box<dyn IGeneRepository>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, new)]
pub struct EnrichmentResult {
    term: String,
    p_value: f64,
    q_value: f64,
}

pub enum EnrichmentAnnotationType {
    Kog,
}

impl GenomeService {
    pub fn new(organism_repository: Box<dyn IOrganismRepository>) -> Self {
        Self {
            organism_repository,
            gene_repositories: HashMap::new(),
        }
    }

    pub fn register_gene_repository(
        &mut self,
        gene_model: GeneModel,
        repository: impl IGeneRepository + 'static,
    ) {
        self.gene_repositories
            .entry(gene_model)
            .or_insert(Box::new(repository));
    }

    pub fn get_gene_repository(&self, gene_model: &GeneModel) -> Option<&Box<dyn IGeneRepository>> {
        self.gene_repositories.get(gene_model)
    }

    pub async fn list_genome_versions(&self, taxonomy_id: u32) -> Result<Vec<GenomeVersion>> {
        let organism = self.organism_repository.get_organism(taxonomy_id).await?;

        Ok(if let Some(o) = organism {
            o.genome_versions
        } else {
            vec![]
        })
    }

    pub async fn enrichment_test(
        &self,
        gene_model: &GeneModel,
        gene_ids: &[&str],
        anntation_type: EnrichmentAnnotationType,
    ) -> Result<Vec<EnrichmentResult>> {
        let repo = self.get_gene_repository(gene_model);

        if let Some(repo) = repo {
            let genes = repo.list_genes(Some(gene_ids)).await?;
            let selected_gene_count = genes.len();
            let total_gene_count = repo.count().await?;

            let mut pvals: Vec<f64> = Vec::new();

            match anntation_type {
                // 1. GenesetのTermを取得
                // 2. Termごとに以下の処理を行う
                // 2a. Genesetの遺伝子数を取得
                // 2b. 全遺伝子数からGenesetの遺伝子数を引いた数を取得
                // 2c.
                // 2d. pvalueを計算
                // 3. FDRを計算
                EnrichmentAnnotationType::Kog => {
                    let kogs: Vec<_> = genes.iter().map(|g| g.kogs()).flatten().collect();

                    for kog in kogs.iter() {
                        let kog_associated_gene_count =
                            repo.count_kog_annotated_genes(kog.id()).await?;
                        let selected_kog_associated_gene_count = genes
                            .iter()
                            .filter(|g| g.kogs().iter().any(|k| k.id() == kog.id()))
                            .count();

                        let not_associated_gene_count =
                            total_gene_count - kog_associated_gene_count;

                        let not_selected_not_associated_gene_count = selected_gene_count
                            - selected_kog_associated_gene_count
                            - not_associated_gene_count;

                        // 要修正
                        let result = fishers_exact(&[
                            selected_kog_associated_gene_count as u32,
                            not_associated_gene_count as u32,
                            selected_gene_count as u32,
                            not_selected_not_associated_gene_count as u32,
                        ])?;

                        pvals.push(result.two_tail_pvalue);
                    }

                    let fdr = multistats_correction::fdr(&pvals);
                    return Ok(kogs
                        .iter()
                        .map(|kog| kog.id().id().to_string())
                        .zip(pvals.iter())
                        .zip(fdr.iter())
                        .map(|((term, pval), qval)| EnrichmentResult::new(term, *pval, *qval))
                        .collect());
                }
            }
        } else {
            return Err(anyhow::anyhow!("Gene repository not found."));
        };
    }
}
