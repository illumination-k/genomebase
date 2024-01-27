use derive_new::new;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use std::str::FromStr;

const DIRECTIVE_PREFIX: &str = "##";

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum GenomeBuildParseError {
    Empty,
    InvalidPrefix(String),
    MissingName(String),
    MissingSource(String),
}

impl fmt::Display for GenomeBuildParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "empty line"),
            Self::InvalidPrefix(line) => write!(f, "invalid prefix: {}", line),
            Self::MissingName(line) => write!(f, "missing name: {}", line),
            Self::MissingSource(line) => write!(f, "missing source: {}", line),
        }
    }
}

impl Error for GenomeBuildParseError {}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, new)]
pub struct GenomeBuild {
    pub name: String,
    pub source: String,
}

impl fmt::Display for GenomeBuild {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}genome-build {} {}",
            DIRECTIVE_PREFIX, self.name, self.source
        )
    }
}

impl FromStr for GenomeBuild {
    type Err = GenomeBuildParseError;

    fn from_str(s: &str) -> Result<Self, GenomeBuildParseError> {
        let mut parts = s.split_whitespace();
        let prefix = parts.next().ok_or_else(|| GenomeBuildParseError::Empty)?;

        if prefix != format!("{}genome-build", DIRECTIVE_PREFIX) {
            return Err(GenomeBuildParseError::InvalidPrefix(prefix.to_string()));
        }

        let name = parts
            .next()
            .ok_or_else(|| GenomeBuildParseError::MissingName("".to_string()))?;
        let source = parts
            .next()
            .ok_or_else(|| GenomeBuildParseError::MissingSource("".to_string()))?;

        Ok(Self::new(name.to_string(), source.to_string()))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, new)]
pub struct SequenceRegion {
    pub seqid: String,
    pub start: u64,
    pub end: u64,
}

impl fmt::Display for SequenceRegion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}sequence-region{} {} {}",
            DIRECTIVE_PREFIX, self.seqid, self.start, self.end
        )
    }
}

impl FromStr for SequenceRegion {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let prefix = parts.next().ok_or_else(|| "missing prefix".to_string())?;

        if prefix != format!("{}sequence-region", DIRECTIVE_PREFIX) {
            return Err(format!("invalid prefix: {}", prefix));
        }

        let seqid = parts
            .next()
            .ok_or_else(|| "missing seqid".to_string())?
            .to_string();

        let start = parts
            .next()
            .ok_or_else(|| "missing start position".to_string())?
            .parse::<u64>()
            .map_err(|e| format!("invalid start position: {}", e))?;

        let end = parts
            .next()
            .ok_or_else(|| "missing end position".to_string())?
            .parse::<u64>()
            .map_err(|e| format!("invalid end position: {}", e))?;

        Ok(Self::new(seqid, start, end))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, new)]
pub struct GffVersion {
    major: u8,
    minor: Option<u8>,
    patch: Option<u8>,
}

impl fmt::Display for GffVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.major, self.minor, self.patch) {
            (major, None, None) => format!("{}gff-version {}.0.0", DIRECTIVE_PREFIX, major).fmt(f),
            (major, Some(minor), None) => {
                format!("{}gff-version{}.{}.0", DIRECTIVE_PREFIX, major, minor).fmt(f)
            }
            (major, Some(minor), Some(patch)) => format!(
                "{}gff-version{}.{}.{}",
                DIRECTIVE_PREFIX, major, minor, patch
            )
            .fmt(f),
            _ => panic!("invalid GFF version"),
        }
    }
}

impl FromStr for GffVersion {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let prefix = parts.next().ok_or_else(|| "missing prefix".to_string())?;

        if prefix != format!("{}gff-version", DIRECTIVE_PREFIX) {
            return Err(format!("invalid prefix: {}", prefix));
        }

        let version = parts.next().ok_or_else(|| "missing version".to_string())?;
        let mut version = version.split('.');
        let major = version
            .next()
            .ok_or_else(|| "missing major version".to_string())?
            .parse::<u8>()
            .map_err(|e| format!("invalid major version: {}", e))?;

        let minor = version
            .next()
            .map(|s| {
                s.parse::<u8>()
                    .map_err(|e| format!("invalid minor version: {}", e))
            })
            .transpose()?;

        let patch = version
            .next()
            .map(|s| {
                s.parse::<u8>()
                    .map_err(|e| format!("invalid patch version: {}", e))
            })
            .transpose()?;

        Ok(Self::new(major, minor, patch))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, new)]
pub struct FeatureOntology {
    pub uri: String,
}

impl fmt::Display for FeatureOntology {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}feature-ontology {}", DIRECTIVE_PREFIX, self.uri)
    }
}

impl FromStr for FeatureOntology {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let prefix = parts.next().ok_or_else(|| "missing prefix".to_string())?;

        if prefix != format!("{}feature-ontology", DIRECTIVE_PREFIX) {
            return Err(format!("invalid prefix: {}", prefix));
        }

        let uri = parts
            .next()
            .ok_or_else(|| "missing URI".to_string())?
            .to_string();

        Ok(Self::new(uri))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, new)]
pub struct AttributeOntology {
    pub uri: String,
}

impl fmt::Display for AttributeOntology {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}attribute-ontology {}", DIRECTIVE_PREFIX, self.uri)
    }
}

impl FromStr for AttributeOntology {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let prefix = parts.next().ok_or_else(|| "missing prefix".to_string())?;

        if prefix != format!("{}attribute-ontology", DIRECTIVE_PREFIX) {
            return Err(format!("invalid prefix: {}", prefix));
        }

        let uri = parts
            .next()
            .ok_or_else(|| "missing URI".to_string())?
            .to_string();

        Ok(Self::new(uri))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, new)]
pub struct SourceOntology {
    pub uri: String,
}

impl fmt::Display for SourceOntology {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}source-ontology {}", DIRECTIVE_PREFIX, self.uri)
    }
}

impl FromStr for SourceOntology {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let prefix = parts.next().ok_or_else(|| "missing prefix".to_string())?;

        if prefix != format!("{}source-ontology", DIRECTIVE_PREFIX) {
            return Err(format!("invalid prefix: {}", prefix));
        }

        let uri = parts
            .next()
            .ok_or_else(|| "missing URI".to_string())?
            .to_string();

        Ok(Self::new(uri))
    }
}

pub const NCBI_TAXONOMY_URI: &str = "http://www.ncbi.nlm.nih.gov/Taxonomy/Browser/wwwtax.cgi?id=";

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, new)]
pub struct Species {
    pub taxon_id: u32,
}

impl fmt::Display for Species {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}species {}{}",
            DIRECTIVE_PREFIX, NCBI_TAXONOMY_URI, self.taxon_id
        )
    }
}

impl FromStr for Species {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let prefix = parts.next().ok_or_else(|| "missing prefix".to_string())?;

        if prefix != format!("{}species", DIRECTIVE_PREFIX) {
            return Err(format!("invalid prefix: {}", prefix));
        }

        let taxon_id = parts
            .next()
            .ok_or_else(|| "missing taxon ID".to_string())?
            .trim_start_matches(NCBI_TAXONOMY_URI)
            .parse::<u32>()
            .map_err(|e| format!("invalid taxon ID: {}", e))?;

        Ok(Self::new(taxon_id))
    }
}

#[cfg(test)]
mod test_standards {
    use super::*;

    #[test]
    fn test_genomebuild_fromstr() {
        let genomebuild = "##genome-build GRCh38.p13 NCBI";
        let genomebuild = GenomeBuild::from_str(genomebuild).unwrap();
        assert_eq!(
            genomebuild,
            GenomeBuild::new("GRCh38.p13".to_string(), "NCBI".to_string())
        );
    }

    #[test]
    fn test_sequence_region_fromstr() {
        let sequence_region = "##sequence-region NC_000001.11 1 248956422";
        let sequence_region = SequenceRegion::from_str(sequence_region).unwrap();
        assert_eq!(
            sequence_region,
            SequenceRegion::new("NC_000001.11".to_string(), 1, 248956422)
        );
    }

    #[test]
    fn test_gff_version_fromstr() {
        let gff_version = "##gff-version 3.1.26";
        let gff_version = GffVersion::from_str(gff_version).unwrap();
        assert_eq!(gff_version, GffVersion::new(3, Some(1), Some(26)));
    }

    #[test]
    fn test_feature_ontology_fromstr() {
        let feature_ontology = "##feature-ontology http://purl.obolibrary.org/obo/so.obo";
        let feature_ontology = FeatureOntology::from_str(feature_ontology).unwrap();
        assert_eq!(
            feature_ontology,
            FeatureOntology::new("http://purl.obolibrary.org/obo/so.obo".to_string())
        );
    }

    #[test]
    fn test_attribute_ontology_fromstr() {
        let attribute_ontology = "##attribute-ontology http://purl.obolibrary.org/obo/gmo.obo";
        let attribute_ontology = AttributeOntology::from_str(attribute_ontology).unwrap();
        assert_eq!(
            attribute_ontology,
            AttributeOntology::new("http://purl.obolibrary.org/obo/gmo.obo".to_string())
        );
    }

    #[test]
    fn test_source_ontology_fromstr() {
        let source_ontology = "##source-ontology http://purl.obolibrary.org/obo/so.obo";
        let source_ontology = SourceOntology::from_str(source_ontology).unwrap();
        assert_eq!(
            source_ontology,
            SourceOntology::new("http://purl.obolibrary.org/obo/so.obo".to_string())
        );
    }

    #[test]
    fn test_species_fromstr() {
        let species = "##species http://www.ncbi.nlm.nih.gov/Taxonomy/Browser/wwwtax.cgi?id=9606";
        let species = Species::from_str(species).unwrap();
        assert_eq!(species, Species::new(9606));
    }
}
