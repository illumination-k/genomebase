use std::collections::HashMap;

use derive_new::new;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, new)]
pub struct Paper {
    doi: String,
    authors: Vec<String>,
    citaion_styles: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, new)]
pub struct User {
    name: String,
    email: String,
    orc_id: String,
}
