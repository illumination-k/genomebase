use derive_new::new;
use serde::{Deserialize, Serialize};

mod go_term;

pub mod kog;

mod kegg;

#[derive(Debug, Clone, Serialize, Deserialize, new)]
pub struct FunctionalAnnotation {
    pub kog: Option<kog::Kog>,
}

impl FunctionalAnnotation {
    pub fn kog(&self) -> &Option<kog::Kog> {
        &self.kog
    }
}
