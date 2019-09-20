use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::{env, io};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

extern "C" {
    pub fn RunQuery(q: *const c_char) -> *const c_char;
}

#[derive(Deserialize)]
struct QueryRequest {
    statement: String,
}

fn query_service(request: web::Json<QueryRequest>) -> impl Responder {
    println!("Query: {:?}", request.statement);

    let query = CString::new(request.statement.clone()).unwrap();
    unsafe {
        let result = CStr::from_ptr(RunQuery(query.as_ptr()));
        HttpResponse::Ok().body(result.to_str().unwrap())
    }
}

fn main() {
    HttpServer::new(|| {
        App::new()
            .route("/query/service", web::post().to(query_service))
        })
        .bind("127.0.0.1:9093")
        .unwrap()
        .run()
        .unwrap();
}

