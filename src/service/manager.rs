use crate::service::Service;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use slog::Logger;

pub struct ManagerService {
    port: usize,
    logger: Logger,
}

impl ManagerService {
    pub fn new(port: usize, logger: Logger) -> Self {
        ManagerService { port, logger }
    }
}

fn pools() -> impl Responder {
    HttpResponse::Ok()
        .content_type("application/json")
        .body("{}")
}

impl Service for ManagerService {
    fn run(&self) {
        HttpServer::new(|| App::new().route("/pools", web::get().to(pools)))
            .bind(format!("127.0.0.1:{}", self.port))
            .unwrap()
            .start();
    }
}
