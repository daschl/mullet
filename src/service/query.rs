use crate::service::Service;
use crate::state::SharedMulletState;
use actix_web::{guard, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use slog::Logger;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::sync::Once;

extern "C" {
    pub fn StartServer();
    pub fn RunQuery(q: *const c_char) -> *const c_char;
}

static INIT_SERVER: Once = Once::new();

#[derive(Deserialize)]
struct QueryRequest {
    statement: String,
}

fn query_service_json(request: web::Json<QueryRequest>) -> impl Responder {
    query_service_internal(request.statement.clone())
}

fn query_service_form(request: web::Form<QueryRequest>) -> impl Responder {
    query_service_internal(request.statement.clone())
}

fn query_service_internal(statement: String) -> impl Responder {
    // println!("Query: {:?}", statement);

    let query = CString::new(statement).unwrap();
    unsafe {
        let result = CStr::from_ptr(RunQuery(query.as_ptr()));
        HttpResponse::Ok().body(result.to_str().unwrap())
    }
}

pub struct QueryService {
    port: usize,
    logger: Logger,
    state: SharedMulletState,
}

impl QueryService {
    pub fn new(port: usize, logger: Logger, state: SharedMulletState) -> Self {
        QueryService {
            port,
            logger,
            state,
        }
    }
}

impl Service for QueryService {
    fn run(&self) {
        INIT_SERVER.call_once(|| unsafe {
            StartServer();
        });

        HttpServer::new(|| {
            App::new()
                .route(
                    "/query/service",
                    web::post()
                        .guard(guard::Header("content-type", "application/json"))
                        .to(query_service_json),
                )
                .route(
                    "/query/service",
                    web::post()
                        .guard(guard::Header(
                            "content-type",
                            "application/x-www-form-urlencoded",
                        ))
                        .to(query_service_form),
                )
        })
        .bind(format!("127.0.0.1:{}", self.port))
        .unwrap()
        .start();
    }
}
