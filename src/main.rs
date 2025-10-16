use oca_repository::configuration::get_configuration;
use oca_repository::logging::{init_tracing, LogOutput};
use oca_repository::startup::run;
use oca_sdk_rs::overlay_registry::OverlayLocalRegistry;
use tracing::info;
use std::net::TcpListener;

// use meilisearch_sdk::client::*;
use oca_sdk_rs::{
    FileSystemStorage, FileSystemStorageConfig, SledDataStorage, SledDataStorageConfig,
};
use oca_sdk_rs::SQLiteConfig;
use oca_sdk_rs::DataStorage;

#[tokio::main]
async fn main() -> std::io::Result<()> {

    let configuration = get_configuration().expect("Failed to read configuration.");
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );

    let log_output = if configuration.application.log_to_file {
        LogOutput::File(configuration.application.log_path)
    } else {
        LogOutput::Stderr
    };

    init_tracing(log_output);

    info!("Application starting up…");
    let listener = TcpListener::bind(address)?;

    let overlayfile_registry = match OverlayLocalRegistry::from_dir(configuration.application.overlayfile_dir) {
        Ok(registry) => {
            info!("Initialized overlay registry: {:?}", registry);
            registry
        }
        Err(e) => {
            eprintln!("Failed to initialize overlay registry: {}", e);
            std::process::exit(1);
        }
    };

    let db_path = std::path::PathBuf::from(configuration.database.path);
    let sled_db = Box::new(
        SledDataStorage::new().config(SledDataStorageConfig::build().path(db_path).unwrap()),
    );
    let filesystem_storage_path = std::path::PathBuf::from(configuration.cache_storage.path);
    let filesystem_storage = Box::new(
        FileSystemStorage::new().config(
            FileSystemStorageConfig::build()
                .path(filesystem_storage_path)
                .unwrap(),
        ),
    );
    let ocafiles_cache_path = std::path::PathBuf::from(configuration.ocafiles_cache.path);
    /* let meilisearch_client = Box::new(Client::new(
        &configuration.search_engine.url,
        &configuration.search_engine.api_key,
    )); */
    let cache_db_path = std::path::PathBuf::from(configuration.search_engine.path);
    let cache_storage_config = SQLiteConfig::build().path(cache_db_path).unwrap();
    let ocafiles_cache = oca_repository::cache::OCAFilesCache::new(ocafiles_cache_path).unwrap();

    run(listener, sled_db, filesystem_storage, cache_storage_config, ocafiles_cache, overlayfile_registry)?.await?;
    Ok(())
}
