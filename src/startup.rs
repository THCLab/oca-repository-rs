use oca_rs::{data_storage::DataStorage, repositories::SQLiteConfig};
use crate::routes::health_check;
// use crate::routes::namespaces;
use crate::routes::oca_bundles;
use std::sync::Arc;

use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
// use actix_web::{dev::ServiceRequest, Error, HttpMessage};
// use actix_web_httpauth::{extractors::{bearer::{BearerAuth}}, middleware::HttpAuthentication};
use std::net::TcpListener;

// use meilisearch_sdk::client::*;

/* async fn validator(
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
} */

pub fn run(
    listener: TcpListener,
    data_storage: Box<dyn DataStorage + Send>,
    cache_storage_config: SQLiteConfig,
) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        // let auth = HttpAuthentication::bearer(validator);
        let data_storage_arc: Arc<Box<dyn DataStorage>> = Arc::new(data_storage.clone());
        let cache_storage_config_arc: Arc<SQLiteConfig> = Arc::new(cache_storage_config.clone());
        let oca_bundles_scope = web::scope("/oca-bundles")
            .route("", web::post().to(oca_bundles::add_oca_file))
            .route("/search", web::get().to(oca_bundles::search))
            .route("/{said}", web::get().to(oca_bundles::get_oca_bundle))
            .route(
                "/{said}/steps",
                web::get().to(oca_bundles::get_oca_file_history),
            )
            .route("/{said}/ocafile", web::get().to(oca_bundles::get_oca_file));
        #[cfg(feature = "data_entries_xls")]
        let oca_bundles_scope = oca_bundles_scope.route(
            "/{said}/data-entry",
            web::get().to(oca_bundles::get_oca_data_entry),
        );
        App::new()
            .app_data(web::Data::from(data_storage_arc))
            .app_data(web::Data::from(cache_storage_config_arc))
            .route("/health_check", web::get().to(health_check))
            /* .route("/namespaces", web::post().to(namespaces::add_namespace))
            .service(
                web::scope("/namespaces/{namespace}")
                    .wrap(auth)
                    .route("", web::get().to(namespaces::get_namespace))
                    .route(
                        "/bundles",
                        web::post().to(namespaces::add_bundle),
                    )
            ) */
            .service(oca_bundles_scope)
        // .route("/search", web::get().to(namespaces::search_bundle))
    })
    .listen(listener)?
    .workers(1)
    .run();

    Ok(server)
}
