#[macro_use]
extern crate log;
extern crate env_logger;

use actix_web::{web, App, HttpServer, Responder};
//use log::Level;

fn index(info: web::Path<(String, u32)>) -> impl Responder {
    debug!("ejs this is a debug {}", "message");
    format!("Hello {}! id:{}", info.0, info.1)
}

fn main() -> std::io::Result<()> {
    env_logger::init();
    HttpServer::new(|| App::new().service(web::resource("/{name}/{id}/hello").to(index)))
        .bind("127.0.0.1:8080")?
        .run()
}
