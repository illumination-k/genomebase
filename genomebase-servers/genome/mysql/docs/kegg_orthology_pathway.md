# kegg_orthology_pathway

## Description

<details>
<summary><strong>Table Definition</strong></summary>

```sql
CREATE TABLE `kegg_orthology_pathway` (
  `kegg_orthology_id` varchar(255) NOT NULL,
  `kegg_pathway_id` varchar(255) NOT NULL,
  PRIMARY KEY (`kegg_orthology_id`,`kegg_pathway_id`),
  KEY `kegg_pathway_id` (`kegg_pathway_id`),
  CONSTRAINT `kegg_orthology_pathway_ibfk_1` FOREIGN KEY (`kegg_orthology_id`) REFERENCES `kegg_orthologies` (`id`),
  CONSTRAINT `kegg_orthology_pathway_ibfk_2` FOREIGN KEY (`kegg_pathway_id`) REFERENCES `kegg_pathways` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci
```

</details>

## Columns

| Name | Type | Default | Nullable | Children | Parents | Comment |
| ---- | ---- | ------- | -------- | -------- | ------- | ------- |
| kegg_orthology_id | varchar(255) |  | false |  | [kegg_orthologies](kegg_orthologies.md) |  |
| kegg_pathway_id | varchar(255) |  | false |  | [kegg_pathways](kegg_pathways.md) |  |

## Constraints

| Name | Type | Definition |
| ---- | ---- | ---------- |
| kegg_orthology_pathway_ibfk_1 | FOREIGN KEY | FOREIGN KEY (kegg_orthology_id) REFERENCES kegg_orthologies (id) |
| kegg_orthology_pathway_ibfk_2 | FOREIGN KEY | FOREIGN KEY (kegg_pathway_id) REFERENCES kegg_pathways (id) |
| PRIMARY | PRIMARY KEY | PRIMARY KEY (kegg_orthology_id, kegg_pathway_id) |

## Indexes

| Name | Definition |
| ---- | ---------- |
| kegg_pathway_id | KEY kegg_pathway_id (kegg_pathway_id) USING BTREE |
| PRIMARY | PRIMARY KEY (kegg_orthology_id, kegg_pathway_id) USING BTREE |

## Relations

![er](kegg_orthology_pathway.svg)

---

> Generated by [tbls](https://github.com/k1LoW/tbls)