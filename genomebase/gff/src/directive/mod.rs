mod standards;
pub use standards::*;

use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

use derive_new::new;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, new)]
pub enum Directive {
    GffVersion(GffVersion),
    Species(Species),
    GenomeBuild(GenomeBuild),
    SequenceRegion(SequenceRegion),
    FeatureOntology(FeatureOntology),
    AttributeOntology(AttributeOntology),
    SourceOntology(SourceOntology),
    ForwardReferencesAreResolved,
    StartOfFasta,
}

impl Display for Directive {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::GffVersion(version) => write!(f, "{}", version),
            Self::Species(species) => write!(f, "{}", species),
            Self::GenomeBuild(build) => write!(f, "{}", build),
            Self::SequenceRegion(region) => write!(f, "{}", region),
            Self::FeatureOntology(ontology) => write!(f, "{}", ontology),
            Self::AttributeOntology(ontology) => write!(f, "{}", ontology),
            Self::SourceOntology(ontology) => write!(f, "{}", ontology),
            Self::ForwardReferencesAreResolved => write!(f, "###"),
            Self::StartOfFasta => write!(f, "##FASTA"),
        }
    }
}

impl FromStr for Directive {
    type Err = Box<dyn std::error::Error>;

    fn from_str(line: &str) -> Result<Directive, Self::Err> {
        let directive = match line {
            line if line.starts_with("##gff-version") => {
                Directive::GffVersion(line.parse::<GffVersion>()?)
            }
            line if line.starts_with("##species") => Directive::Species(line.parse::<Species>()?),
            line if line.starts_with("##genome-build") => {
                Directive::GenomeBuild(line.parse::<GenomeBuild>()?)
            }
            line if line.starts_with("##sequence-region") => {
                Directive::SequenceRegion(line.parse::<SequenceRegion>()?)
            }
            line if line.starts_with("##feature-ontology") => {
                Directive::FeatureOntology(line.parse::<FeatureOntology>()?)
            }
            line if line.starts_with("##attribute-ontology") => {
                Directive::AttributeOntology(line.parse::<AttributeOntology>()?)
            }
            line if line.starts_with("##source-ontology") => {
                Directive::SourceOntology(line.parse::<SourceOntology>()?)
            }
            line if line.starts_with("###") => Directive::ForwardReferencesAreResolved,
            line if line.starts_with("##FASTA") => Directive::StartOfFasta,
            _ => return Err(format!("invalid directive: {}", line).into()),
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
