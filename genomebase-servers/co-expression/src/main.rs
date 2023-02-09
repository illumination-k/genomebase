use std::collections::HashMap;

use anyhow::Result;

use mongodb::{options::ClientOptions, Client};

mod mongo;

#[tokio::main]
async fn main() -> Result<()> {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await?;

    let client = Client::with_options(client_options)?;
    let db = client.database("co-expression");

    let collection = db.collection::<mongo::CorrealtionDocument>("test");

    Ok(())
}
