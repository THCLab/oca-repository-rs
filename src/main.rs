use oca_repository::configuration::get_configuration;
use oca_repository::startup::run;
use std::net::TcpListener;

// use meilisearch_sdk::client::*;
use oca_rs::data_storage::{DataStorage, SledDataStorage, SledDataStorageConfig};
use oca_rs::repositories::SQLiteConfig;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address)?;

    let sled_db = Box::new(
        SledDataStorage::new()
            .config(
                SledDataStorageConfig::build()
                    .path(configuration.database.path)
                    .unwrap()
            )
    );
    /* let meilisearch_client = Box::new(Client::new(
        &configuration.search_engine.url,
        &configuration.search_engine.api_key,
    )); */
    let cache_storage_config = SQLiteConfig::build().path(
        configuration.search_engine.path
    ).unwrap();

    run(listener, sled_db, cache_storage_config)?.await?;
    Ok(())
}
