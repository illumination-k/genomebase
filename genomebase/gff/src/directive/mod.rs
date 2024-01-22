mod standards;
pub use standards::*;

use anyhow::Result;
use derive_new::new;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, new)]
pub enum Directive {
    GffVersion,
    Species,
    GenomeBuild,
    SequenceRegion,
    FeatureOntology,
    AttributeOntology,
    SourceOntology,
    ForwardReferencesAreResolved,
    StartOfFasta,
}

impl Directive {
    pub fn from_line(line: &str) -> Result<Directive> {
        let directive = match line {
            line if line.starts_with("##gff-version") => Directive::GffVersion,
            line if line.starts_with("##species") => Directive::Species,
            line if line.starts_with("##genome-build") => Directive::GenomeBuild,
            line if line.starts_with("##sequence-region") => Directive::SequenceRegion,
            line if line.starts_with("##feature-ontology") => Directive::FeatureOntology,
            line if line.starts_with("##attribute-ontology") => Directive::AttributeOntology,
            line if line.starts_with("##source-ontology") => Directive::SourceOntology,
            line if line.starts_with("###") => Directive::ForwardReferencesAreResolved,
            line if line.starts_with("##FASTA") => Directive::StartOfFasta,
            _ => anyhow::bail!("invalid directive"),
        };

        Ok(directive)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, new)]
pub struct DirectiveHeader {
    version: GffVersion,
    species: Species,
    genome_build: GenomeBuild,
    sequence_region: Vec<SequenceRegion>,
    feature_ontology: Vec<FeatureOntology>,
    attribute_ontology: Vec<AttributeOntology>,
    source_ontology: Vec<SourceOntology>,
}

impl DirectiveHeader {
    pub fn format_as_header(&self) -> String {
        let mut header: Vec<String> = Vec::new();

        header.push(self.version.to_string());
        header.push(self.species.to_string());
        header.push(self.genome_build.to_string());

        for region in &self.sequence_region {
            header.push(region.to_string());
        }

        for ontology in &self.feature_ontology {
            header.push(ontology.to_string());
        }

        for ontology in &self.attribute_ontology {
            header.push(ontology.to_string());
        }

        for ontology in &self.source_ontology {
            header.push(ontology.to_string());
        }

        header.join("\n")
    }
}
