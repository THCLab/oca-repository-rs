use oca_repository::configuration::get_configuration;
use oca_repository::startup::run;
use std::net::TcpListener;

use meilisearch_sdk::client::*;
use oca_rs::data_storage::{DataStorage, SledDataStorage};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address)?;

    let sled_db = Box::new(SledDataStorage::open(&configuration.database.path));
    let meilisearch_client = Box::new(Client::new(
        &configuration.search_engine.url,
        &configuration.search_engine.api_key,
    ));

    run(listener, sled_db, meilisearch_client)?.await?;
    Ok(())
}
