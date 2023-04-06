-- Add up migration script here
-- User
CREATE TABLE IF NOT EXISTS users  (
    id BINARY(16) NOT NULL PRIMARY KEY DEFAULT (UUID_TO_BIN(UUID())),
    orc_id VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL
);

-- Organismテーブル
CREATE TABLE IF NOT EXISTS organisms (
    taxonomy_id INT UNSIGNED NOT NULL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    codon_code INT UNSIGNED NOT NULL DEFAULT 1
);

-- Genomeテーブル
CREATE TABLE IF NOT EXISTS genomes (
    id BINARY(16) NOT NULL PRIMARY KEY DEFAULT (UUID_TO_BIN(UUID())),
    genome_version VARCHAR(255) NOT NULL,
    organism_taxonomy_id INT UNSIGNED NOT NULL,
    FOREIGN KEY (organism_taxonomy_id) REFERENCES organisms(taxonomy_id),
    
    created_at DATETIME DEFAULT current_timestamp,
    updated_at DATETIME DEFAULT current_timestamp ON UPDATE current_timestamp
);

-- AnnotationModelテーブル
CREATE TABLE IF NOT EXISTS annotation_models (
    id BINARY(16) NOT NULL PRIMARY KEY DEFAULT (UUID_TO_BIN(UUID())),
    version VARCHAR(255) NOT NULL,
    genome_id BINARY(16) NOT NULL,
    FOREIGN KEY (genome_id) REFERENCES genomes(id),

    created_at DATETIME DEFAULT current_timestamp,
    updated_at DATETIME DEFAULT current_timestamp ON UPDATE current_timestamp
);

CREATE TABLE IF NOT EXISTS gene_models (
    id BINARY(16) NOT NULL PRIMARY KEY DEFAULT (UUID_TO_BIN(UUID())),
    taxonomy_id INT UNSIGNED NOT NULL,
    FOREIGN KEY (taxonomy_id) REFERENCES organisms(taxonomy_id),

    genome_id BINARY(16) NOT NULL,
    FOREIGN KEY (genome_id) REFERENCES genomes(id),
    
    annotation_model_id BINARY(16) NOT NULL,
    FOREIGN KEY (annotation_model_id) REFERENCES annotation_models(id)
);

CREATE TABLE IF NOT EXISTS genes (
    id BINARY(16) NOT NULL PRIMARY KEY DEFAULT (UUID_TO_BIN(UUID())),
    gene_id VARCHAR(255) NOT NULL,
    gene_model_id BINARY(16) NOT NULL,
    UNIQUE (gene_id, gene_model_id),
    FOREIGN KEY (gene_model_id) REFERENCES gene_models(id)
);

-- Transcriptテーブル
CREATE TABLE IF NOT EXISTS transcripts (
    id BINARY(16) NOT NULL PRIMARY KEY DEFAULT (UUID_TO_BIN(UUID())),
    transcript_id VARCHAR(255) NOT NULL,
    gene_model_id BINARY(16) NOT NULL,
    gene_id BINARY(16) NOT NULL,
    transcript_type VARCHAR(255) NOT NULL,
    start INT NOT NULL,
    end INT NOT NULL,
    strand CHAR(1) NOT NULL,
    
    -- structureは別のテーブルで管理
    -- annotationは別のテーブルで管理
    UNIQUE (transcript_id, gene_model_id),
    FOREIGN KEY (gene_model_id) REFERENCES gene_models(id),
    FOREIGN KEY (gene_id) REFERENCES genes(id)
);

-- Functional Annotation Tables

-- Kogテーブル
CREATE TABLE IF NOT EXISTS kogs (
    id VARCHAR(255) NOT NULL PRIMARY KEY,
    category ENUM('A', 'K', 'L', 'B', 'J', 'D', 'Y', 'V', 'T', 'M', 'N', 'Z', 'W', 'U', 'O', 'E', 'F', 'H', 'I', 'G', 'P', 'C', 'Q', 'R', 'S', 'X') NOT NULL,
    description TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS kog_annotations (
    kog_id VARCHAR(255) NOT NULL,
    transcript_id BINARY(16) NOT NULL,

    PRIMARY KEY (kog_id, transcript_id),
    FOREIGN KEY (kog_id) REFERENCES kogs(id),
    FOREIGN KEY (transcript_id) REFERENCES transcripts(id)
);

-- Go Term Table
CREATE TABLE IF NOT EXISTS go_terms (
    id VARCHAR(255) NOT NULL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    namespace ENUM("BP", "CC", "MF") NOT NULL,
    definition TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS go_terms_annotation (
    go_term_id VARCHAR(255) NOT NULL,
    transcript_id BINARY(16) NOT NULL,
    evidence_code CHAR(3) NOT NULL,
    
    PRIMARY KEY (go_term_id, transcript_id),
    FOREIGN KEY (go_term_id) REFERENCES go_terms(id),
    FOREIGN KEY (transcript_id) REFERENCES transcripts(id),

    -- go term can be annotated manually or automatically
    -- evidence code is used to distinguish them
    -- annotation status columns
    created_at DATETIME DEFAULT current_timestamp,
    updated_at DATETIME DEFAULT current_timestamp ON UPDATE current_timestamp,
    assisgned_by BINARY(16) NULL,
    FOREIGN KEY (assisgned_by) REFERENCES users(id)
);

-- Domain Annotation Table
CREATE TABLE IF NOT EXISTS domains (
    id VARCHAR(255) NOT NULL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS domain_annotations (
    domain_id VARCHAR(255) NOT NULL,
    transcript_id BINARY(16) NOT NULL,
    start INT NOT NULL,
    end INT NOT NULL,
    PRIMARY KEY (domain_id, transcript_id),
    FOREIGN KEY (domain_id) REFERENCES domains(id),
    FOREIGN KEY (transcript_id) REFERENCES transcripts(id)
);

-- Kegg Tables

CREATE TABLE IF NOT EXISTS kegg_orthologies (
    id VARCHAR(255) NOT NULL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS kegg_pathways (
    id VARCHAR(255) NOT NULL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS kegg_reaction (
    id VARCHAR(255) NOT NULL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS kegg_orthologies_annotation (
    kegg_orthology_id VARCHAR(255) NOT NULL,
    transcript_id BINARY(16) NOT NULL,
    PRIMARY KEY (kegg_orthology_id, transcript_id),
    FOREIGN KEY (kegg_orthology_id) REFERENCES kegg_orthologies(id),
    FOREIGN KEY (transcript_id) REFERENCES transcripts(id),

    created_at DATETIME DEFAULT current_timestamp,
    updated_at DATETIME DEFAULT current_timestamp ON UPDATE current_timestamp
);

CREATE TABLE IF NOT EXISTS kegg_pathways_annotation (
    kegg_pathway_id VARCHAR(255) NOT NULL,
    transcript_id BINARY(16) NOT NULL,
    PRIMARY KEY (kegg_pathway_id, transcript_id),
    FOREIGN KEY (kegg_pathway_id) REFERENCES kegg_pathways(id),
    FOREIGN KEY (transcript_id) REFERENCES transcripts(id),

    created_at DATETIME DEFAULT current_timestamp,
    updated_at DATETIME DEFAULT current_timestamp ON UPDATE current_timestamp
);

CREATE TABLE IF NOT EXISTS kegg_reaction_annotation (
    kegg_reaction_id VARCHAR(255) NOT NULL,
    transcript_id BINARY(16) NOT NULL,

    PRIMARY KEY (kegg_reaction_id, transcript_id),
    FOREIGN KEY (kegg_reaction_id) REFERENCES kegg_reaction(id),
    FOREIGN KEY (transcript_id) REFERENCES transcripts(id),

    created_at DATETIME DEFAULT current_timestamp,
    updated_at DATETIME DEFAULT current_timestamp ON UPDATE current_timestamp
);

-- # GffRecord Tables
CREATE TABLE gff_records (
    id BINARY(16) NOT NULL PRIMARY KEY DEFAULT (UUID_TO_BIN(UUID())),
    gene_model_id BINARY(16) NOT NULL,

    seqname VARCHAR(255) NOT NULL,
    source VARCHAR(255) NOT NULL,
    type VARCHAR(255) NOT NULL,
    start INT NOT NULL,
    end INT NOT NULL,
    score FLOAT NOT NULL,
    strand CHAR(1) NOT NULL,
    phase CHAR(1) NOT NULL,
    attributes JSON NOT NULL,
    FOREIGN KEY (gene_model_id) REFERENCES gene_models(id)
);

CREATE INDEX idx_gff_records_seqname_start_end ON gff_records (seqname, start, end);

-- TranscriptStructureテーブル
CREATE TABLE IF NOT EXISTS transcript_structure (
    transcript_id BINARY(16) NOT NULL,
    gff_record_id BINARY(16) NOT NULL,
    PRIMARY KEY (transcript_id, gff_record_id),
    FOREIGN KEY (transcript_id) REFERENCES transcripts(id),
    FOREIGN KEY (gff_record_id) REFERENCES gff_records(id)
);
