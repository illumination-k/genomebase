pub mod import;
pub mod repositories;

pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");
