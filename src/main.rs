use actix_web::{web, App, HttpResponse, HttpServer, Responder, guard};
use serde::Deserialize;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

extern "C" {
    pub fn StartServer();
    pub fn RunQuery(q: *const c_char) -> *const c_char;
}

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

fn main() {
    unsafe {
        StartServer();
    }

    HttpServer::new(|| App::new()
        .route(
        "/query/service",
        web::post()
            .guard(guard::Header("content-type", "application/json"))
            .to(query_service_json))
        .route(
        "/query/service",
        web::post()
            .guard(guard::Header("content-type", "application/x-www-form-urlencoded"))
            .to(query_service_form))
    )
    .bind("127.0.0.1:9093")
    .unwrap()
    .run()
    .unwrap();
}
