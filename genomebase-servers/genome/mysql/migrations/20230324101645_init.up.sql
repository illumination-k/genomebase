-- Add up migration script here
-- User
CREATE TABLE IF NOT EXISTS users  (
    id BINARY(16) NOT NULL PRIMARY KEY DEFAULT (UUID_TO_BIN(UUID())),
    orc_id VARCHAR(255) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE
);

-- Organismテーブル
CREATE TABLE IF NOT EXISTS organisms (
    taxonomy_id INT UNSIGNED NOT NULL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    codon_code INT UNSIGNED NOT NULL DEFAULT 1
);

-- Genome version management
-- Major version: assigned when a new version of the genomic sequence is released, which may include biologically significant changes
-- Minor version: assigned when non-destructive changes are made, such as computational corrections
-- Patch version: assigned when minor modifications are made, such as small sequence corrections using Sanger sequencing

CREATE TABLE IF NOT EXISTS genome_versions (
    id BINARY(16) NOT NULL PRIMARY KEY DEFAULT (UUID_TO_BIN(UUID())),
    taxonomy_id INT UNSIGNED NOT NULL,
    FOREIGN KEY (taxonomy_id) REFERENCES organisms(taxonomy_id),
    
    major_version INT NOT NULL,
    minor_version INT NOT NULL,
    patch_version INT NOT NULL,

    fasta_uri VARCHAR(255) NOT NULL,
    fasta_index_uri VARCHAR(255) NOT NULL,
    UNIQUE (taxonomy_id, major_version, minor_version, patch_version)
);

-- Annotation version management
-- Major version: may include the addition or deletion of gene/transcript IDs
-- Minor version: assigned when non-destructive changes are made, such as changes
-- Patch version: assigned when minor modifications are made, excluding changes in gene/transcript IDs

CREATE TABLE IF NOT EXISTS annotation_versions (
    id BINARY(16) NOT NULL PRIMARY KEY DEFAULT (UUID_TO_BIN(UUID())),
    genome_version_id BINARY(16) NOT NULL,
    major_version INT NOT NULL,
    minor_version INT NOT NULL,
    patch_version INT NOT NULL,
    FOREIGN KEY (genome_version_id) REFERENCES genome_versions(id),
    UNIQUE (genome_version_id, major_version, minor_version, patch_version)
);

CREATE TABLE IF NOT EXISTS genes (
    id BINARY(16) NOT NULL PRIMARY KEY DEFAULT (UUID_TO_BIN(UUID())),
    gene_id VARCHAR(255) NOT NULL,
    annotation_version_id BINARY(16) NOT NULL,
    FOREIGN KEY (annotation_version_id) REFERENCES annotation_versions(id)
);

CREATE TABLE IF NOT EXISTS nomenclatures (
    id BINARY(16) NOT NULL PRIMARY KEY DEFAULT (UUID_TO_BIN(UUID())),
    gene_id BINARY(16) NOT NULL,
    FOREIGN KEY (gene_id) REFERENCES genes(id),

    name VARCHAR(255) NOT NULL,
    doi VARCHAR(255) NULL,
    citation TEXT NULL,
    status ENUM('obsolate', 'provisional', 'published') NOT NULL,

    assigned_by BINARY(16) NULL,
    FOREIGN KEY (assigned_by) REFERENCES users(id),
    created_at DATETIME DEFAULT current_timestamp,
    updated_at DATETIME DEFAULT current_timestamp ON UPDATE current_timestamp
);

CREATE TABLE IF NOT EXISTS gene_cross_references (
    source_gene_id BINARY(16) NOT NULL,
    source_annotation_version_id BINARY(16) NOT NULL,
    target_gene_id BINARY(16) NOT NULL,
    target_annotation_version_id BINARY(16) NOT NULL,
    PRIMARY KEY (source_gene_id, source_annotation_version_id, target_gene_id, target_annotation_version_id),
    FOREIGN KEY (source_gene_id) REFERENCES genes(id),
    FOREIGN KEY (source_annotation_version_id) REFERENCES annotation_versions(id),
    FOREIGN KEY (target_gene_id) REFERENCES genes(id),
    FOREIGN KEY (target_annotation_version_id) REFERENCES annotation_versions(id)
);

-- Transcriptテーブル
CREATE TABLE IF NOT EXISTS transcripts (
    id BINARY(16) NOT NULL PRIMARY KEY DEFAULT (UUID_TO_BIN(UUID())),
    transcript_id VARCHAR(255) NOT NULL,
    transcript_type VARCHAR(255) NOT NULL,
    start INT NOT NULL,
    end INT NOT NULL,
    strand CHAR(1) NOT NULL,
    
    annotation_version_id BINARY(16) NOT NULL,
    FOREIGN KEY (annotation_version_id) REFERENCES annotation_versions(id),

    gene_id BINARY(16) NOT NULL,
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
    definition TEXT NOT NULL,

    parent_id VARCHAR(255) NULL,
    FOREIGN KEY (parent_id) REFERENCES go_terms(id)
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
    assigned_by BINARY(16) NULL,
    FOREIGN KEY (assigned_by) REFERENCES users(id)
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

-- Many-to-many relationship table between Kegg Orthologies and Kegg Pathways
CREATE TABLE IF NOT EXISTS kegg_orthology_pathway (
    kegg_orthology_id VARCHAR(255) NOT NULL,
    kegg_pathway_id VARCHAR(255) NOT NULL,
    PRIMARY KEY (kegg_orthology_id, kegg_pathway_id),
    FOREIGN KEY (kegg_orthology_id) REFERENCES kegg_orthologies(id),
    FOREIGN KEY (kegg_pathway_id) REFERENCES kegg_pathways(id)
);

-- Self many-to-many relationship table for Kegg Pathways
CREATE TABLE IF NOT EXISTS kegg_pathway_relations (
    parent_pathway_id VARCHAR(255) NOT NULL,
    child_pathway_id VARCHAR(255) NOT NULL,
    PRIMARY KEY (parent_pathway_id, child_pathway_id),
    FOREIGN KEY (parent_pathway_id) REFERENCES kegg_pathways(id),
    FOREIGN KEY (child_pathway_id) REFERENCES kegg_pathways(id)
);

-- Many-to-many relationship table between Kegg Reaction and Kegg Orthologies
CREATE TABLE IF NOT EXISTS kegg_reaction_orthology (
    kegg_reaction_id VARCHAR(255) NOT NULL,
    kegg_orthology_id VARCHAR(255) NOT NULL,
    PRIMARY KEY (kegg_reaction_id, kegg_orthology_id),
    FOREIGN KEY (kegg_reaction_id) REFERENCES kegg_reaction(id),
    FOREIGN KEY (kegg_orthology_id) REFERENCES kegg_orthologies(id)
);

-- Many-to-many relationship table between Kegg Reaction and Kegg Pathways
CREATE TABLE IF NOT EXISTS kegg_reaction_pathway (
    kegg_reaction_id VARCHAR(255) NOT NULL,
    kegg_pathway_id VARCHAR(255) NOT NULL,
    PRIMARY KEY (kegg_reaction_id, kegg_pathway_id),
    FOREIGN KEY (kegg_reaction_id) REFERENCES kegg_reaction(id),
    FOREIGN KEY (kegg_pathway_id) REFERENCES kegg_pathways(id)
);

-- # GffRecord Tables
CREATE TABLE gff_records (
    id BINARY(16) NOT NULL PRIMARY KEY DEFAULT (UUID_TO_BIN(UUID())),
    annotation_version_id BINARY(16) NOT NULL,
    FOREIGN KEY (annotation_version_id) REFERENCES annotation_versions(id),

    seqname VARCHAR(255) NOT NULL,
    source VARCHAR(255) NOT NULL,
    type VARCHAR(255) NOT NULL,
    start INT NOT NULL,
    end INT NOT NULL,
    score FLOAT NOT NULL,
    strand CHAR(1) NOT NULL,
    phase CHAR(1) NOT NULL,
    attributes JSON NOT NULL
);

CREATE INDEX index_gff_records_annotation_id_seqname_start_end ON gff_records (annotation_version_id, seqname, start, end);

-- TranscriptStructureテーブル
CREATE TABLE IF NOT EXISTS transcript_structure (
    transcript_id BINARY(16) NOT NULL,
    gff_record_id BINARY(16) NOT NULL,
    PRIMARY KEY (transcript_id, gff_record_id),
    FOREIGN KEY (transcript_id) REFERENCES transcripts(id),
    FOREIGN KEY (gff_record_id) REFERENCES gff_records(id)
);
