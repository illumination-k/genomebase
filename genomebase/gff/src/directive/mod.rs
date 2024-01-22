mod standards;
pub use standards::*;

use derive_new::new;
use serde::{Deserialize, Serialize};

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
