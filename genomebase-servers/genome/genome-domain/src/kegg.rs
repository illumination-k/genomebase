use anyhow::Result;
use paste::paste;
use serde::{Deserialize, Serialize};

use crate::{impl_term_serde, term_id_deserializer, term_id_serializer, TermID};

#[derive(Debug, Clone, PartialEq)]
pub struct OrthologyID(String);

impl TermID for OrthologyID {
    fn try_new(id: &str) -> Result<Self> {
        Ok(Self(id.to_string()))
    }

    fn id(&self) -> String {
        self.0.to_owned()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PathwayID(String);

impl TermID for PathwayID {
    fn try_new(id: &str) -> Result<Self> {
        Ok(Self(id.to_string()))
    }

    fn id(&self) -> String {
        self.0.to_owned()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReactionID(String);

impl TermID for ReactionID {
    fn try_new(id: &str) -> Result<Self> {
        Ok(Self(id.to_string()))
    }

    fn id(&self) -> String {
        self.0.to_owned()
    }
}

impl_term_serde!(ReactionID);
impl_term_serde!(PathwayID);
impl_term_serde!(OrthologyID);

macro_rules! impl_kegg_value {
    ($t: ident) => {
        paste! { #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
            pub struct $t {
                id: [<$t ID>],
                description: String,
            }

            impl $t {
                pub fn try_new(id: &str, description: String) -> Result<Self> {
                    let id = [<$t ID>]::try_new(id)?;
                    Ok(Self { id, description })
                }
            }
        }
    };
}

impl_kegg_value!(Orthology);
impl_kegg_value!(Pathway);
impl_kegg_value!(Reaction);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Annotation {
    orthology: Orthology,
    related_pathways: Vec<Pathway>,
    related_reactions: Vec<Reaction>,
}

#[cfg(test)]
mod test_ {
    use super::*;
    use anyhow::Result;
    use serde_test::{assert_tokens, Token};

    #[test]
    fn test_orthology() -> Result<()> {
        let id = "ko:000001";
        let description = "aaa";

        let orthology = Orthology::try_new(id, description.to_string())?;

        assert_tokens(
            &orthology,
            &[
                Token::Struct {
                    name: "Orthology",
                    len: 2,
                },
                Token::Str("id"),
                Token::Str(id),
                Token::Str("description"),
                Token::Str(description),
                Token::StructEnd,
            ],
        );

        Ok(())
    }
}
