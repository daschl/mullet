use crate::service::Service;
use crate::state::SharedMulletState;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use slog::Logger;

pub struct ManagerService {
    port: usize,
    logger: Logger,
    state: SharedMulletState,
}

impl ManagerService {
    pub fn new(port: usize, logger: Logger, state: SharedMulletState) -> Self {
        ManagerService {
            port,
            logger,
            state,
        }
    }
}

fn pools(state: web::Data<SharedMulletState>) -> impl Responder {
    HttpResponse::Ok()
        .content_type("application/json")
        .body("{}")
}

fn buckets(state: web::Data<SharedMulletState>) -> impl Responder {
    let result = state.lock().unwrap().export_all_bucket_configs(true);
    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&result).unwrap())
}

fn bucket_verbose(path: web::Path<String>, state: web::Data<SharedMulletState>) -> impl Responder {
    let name = &*path;
    let config = state.lock().unwrap().export_bucket_config(name, true);
    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&config).unwrap())
}

fn bucket_terse(path: web::Path<String>, state: web::Data<SharedMulletState>) -> impl Responder {
    let name = &*path;
    let config = state.lock().unwrap().export_bucket_config(name, false);
    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&config).unwrap())
}

impl Service for ManagerService {
    fn run(&self) {
        let state = self.state.clone();
        HttpServer::new(move || {
            App::new()
                .data(state.clone())
                .route("/pools", web::get().to(pools))
                .route("/pools/default/buckets", web::get().to(buckets))
                .route(
                    "/pools/default/buckets/{name}",
                    web::get().to(bucket_verbose),
                )
                .route(
                    "/pools/default/b/{name}",
                    web::get().to(bucket_terse),
                )

        })
        .bind(format!("127.0.0.1:{}", self.port))
        .unwrap()
        .start();
    }
}
