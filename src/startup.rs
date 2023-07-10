use oca_dag::data_storage::DataStorage;
use crate::routes::health_check;
use crate::routes::namespaces;
use std::sync::Arc;

use actix_web::dev::Server;
use actix_web::{dev::ServiceRequest, Error, web, App, HttpServer, HttpMessage};
use actix_web_httpauth::{extractors::{bearer::{BearerAuth}}, middleware::HttpAuthentication};
use std::net::TcpListener;

use meilisearch_sdk::client::*;

async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    req.extensions_mut().insert(credentials.token().to_string());

    let namespace = req.match_info().get("namespace").unwrap();
    let _public_key = req.app_data::<web::Data<Box<dyn DataStorage>>>()
        .unwrap()
        .get_ref()
        .get(&format!("namespace.{namespace}.public_key"));
    Ok(req)
}

pub fn run(
    listener: TcpListener,
    data_storage: Box<dyn DataStorage + Send>,
    search_engine_client: Box<Client>,
) -> Result<Server, std::io::Error> {

    let server = HttpServer::new(move || {
        let auth = HttpAuthentication::bearer(validator);
        let data_storage_arc: Arc<Box<dyn DataStorage>> = Arc::new(data_storage.clone());
        let search_engine_client_arc: Arc<Box<Client>> = Arc::new(search_engine_client.clone());
        App::new()
            .app_data(web::Data::from(data_storage_arc))
            .app_data(web::Data::from(search_engine_client_arc))
            .route("/health_check", web::get().to(health_check))
            .route("/namespaces", web::post().to(namespaces::add_namespace))
            .service(
                web::scope("/namespaces/{namespace}")
                    .wrap(auth)
                    .route("", web::get().to(namespaces::get_namespace))
                    .route(
                        "/bundles",
                        web::post().to(namespaces::add_bundle),
                    )
            )
            .route("/oca-bundle", web::post().to(namespaces::add_oca_file))
            .route("/oca-bundle/{said}", web::get().to(namespaces::get_oca_bundle))
            .route("/oca-bundle/{said}/steps", web::get().to(namespaces::get_oca_file_history))
            .route("/search", web::get().to(namespaces::search_bundle))
    })
    .listen(listener)?
    .workers(1)
    .run();

    Ok(server)
}
