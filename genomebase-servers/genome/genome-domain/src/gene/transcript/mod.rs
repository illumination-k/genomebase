pub mod annotation;

use bio::io::gff::Record as GffRecord;

use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::gene::Strand;

use self::annotation::FunctionalAnnotation;


#[derive(Debug, Clone, Serialize, Deserialize, derive_new::new)]
pub struct Transcript {
    id: Uuid,
    transcript_id: String,
    transcript_type: String,
    start: i32,
    end: i32,
    strand: Strand,
    gff_records: Vec<GffRecord>,
    annotation: FunctionalAnnotation,
}

impl Transcript {
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn transcript_id(&self) -> &String {
        &self.transcript_id
    }

    pub fn transcript_type(&self) -> &String {
        &self.transcript_type
    }

    pub fn start(&self) -> i32 {
        self.start
    }

    pub fn end(&self) -> i32 {
        self.end
    }

    pub fn strand(&self) -> &Strand {
        &self.strand
    }

    pub fn gff_records(&self) -> &Vec<GffRecord> {
        &self.gff_records
    }

    pub fn extend_gff_records(&mut self, gff_records: impl IntoIterator<Item = GffRecord>) {
        self.gff_records.extend(gff_records);
    }
    
    pub fn annotation(&self) -> &FunctionalAnnotation {
        &self.annotation
    }

    pub fn annotation_mut(&mut self) -> &mut FunctionalAnnotation {
        &mut self.annotation
    }
}
