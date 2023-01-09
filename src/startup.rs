use crate::data_storage::DataStorage;
use crate::routes::health_check;
use crate::routes::namespaces;
use std::sync::Arc;

use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;

use meilisearch_sdk::client::*;

pub fn run(
    listener: TcpListener,
    data_storage: Box<dyn DataStorage + Send>,
    search_engine_client: Box<Client>,
) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        let data_storage_arc: Arc<Box<dyn DataStorage>> = Arc::new(data_storage.clone());
        let search_engine_client_arc: Arc<Box<Client>> = Arc::new(search_engine_client.clone());
        App::new()
            .app_data(web::Data::from(data_storage_arc))
            .app_data(web::Data::from(search_engine_client_arc))
            .route("/health_check", web::get().to(health_check))
            .route("/namespaces", web::post().to(namespaces::add_namespace))
            .route(
                "/namespaces/{namespace}/bundles",
                web::post().to(namespaces::add_bundle),
            )
            .route("/search", web::get().to(namespaces::search_bundle))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
