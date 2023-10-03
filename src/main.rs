use oca_repository::configuration::get_configuration;
use oca_repository::startup::run;
use std::net::TcpListener;

// use meilisearch_sdk::client::*;
use oca_rs::data_storage::{
    DataStorage, FileSystemStorage, FileSystemStorageConfig, SledDataStorage,
    SledDataStorageConfig,
};
use oca_rs::repositories::SQLiteConfig;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration =
        get_configuration().expect("Failed to read configuration.");
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address)?;

    let db_path = std::path::PathBuf::from(configuration.database.path);
    let sled_db = Box::new(
        SledDataStorage::new()
            .config(SledDataStorageConfig::build().path(db_path).unwrap()),
    );
    let filesystem_storage_path =
        std::path::PathBuf::from(configuration.cache_storage.path);
    let filesystem_storage = Box::new(
        FileSystemStorage::new().config(
            FileSystemStorageConfig::build()
                .path(filesystem_storage_path)
                .unwrap(),
        ),
    );
    /* let meilisearch_client = Box::new(Client::new(
        &configuration.search_engine.url,
        &configuration.search_engine.api_key,
    )); */
    let cache_db_path =
        std::path::PathBuf::from(configuration.search_engine.path);
    let cache_storage_config =
        SQLiteConfig::build().path(cache_db_path).unwrap();

    run(listener, sled_db, filesystem_storage, cache_storage_config)?.await?;
    Ok(())
}
