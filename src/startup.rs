use crate::cache::OCAFilesCache;
use crate::routes::health_check;
use oca_sdk_rs::overlay_registry::OverlayLocalRegistry;
use oca_sdk_rs::Store;
use oca_sdk_rs::{DataStorage, SQLiteConfig};
// use crate::routes::namespaces;
use crate::routes::{explore, internal, objects, oca_bundles};
use std::sync::{Arc, Mutex};

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

pub struct AppState {
    pub facade: Mutex<Store>,
    pub cache: OCAFilesCache,
    pub overlayfile_registry: OverlayLocalRegistry,
}

pub fn run(
    listener: TcpListener,
    data_storage: Box<dyn DataStorage + Send + Sync>,
    filesystem_storage: Box<dyn DataStorage + Send + Sync>,
    cache_storage_config: SQLiteConfig,
    ocafiles_cache: OCAFilesCache,
    overlayfile_registry: OverlayLocalRegistry,
) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        // let auth = HttpAuthentication::bearer(validator);
        let state = AppState {
            facade: Mutex::new(Store::new(
                data_storage.clone(),
                filesystem_storage.clone(),
                cache_storage_config.clone(),
            )),
            cache: ocafiles_cache.clone(),
            overlayfile_registry: overlayfile_registry.clone(),
        };
        #[allow(clippy::arc_with_non_send_sync)]
        let facade_arc = Arc::new(state);
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
            .app_data(web::Data::from(facade_arc))
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
            .service(web::scope("/objects").route("", web::get().to(objects::fetch_objects)))
            .service(
                web::scope("/explore").route("/{said}", web::get().to(explore::fetch_relations)),
            )
            .service(
                web::scope("/internal")
                    .route(
                        "/capture-bases",
                        web::get().to(internal::fetch_all_capture_base),
                    )
                    .route(
                        "/oca-bundles",
                        web::get().to(internal::fetch_all_oca_bundle),
                    ),
            )
        // .route("/search", web::get().to(namespaces::search_bundle))
    })
    .listen(listener)?
    .workers(1)
    .run();

    Ok(server)
}
