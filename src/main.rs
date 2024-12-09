
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use listenfd::ListenFd;
use std::env;
mod schema;
mod error_handler;
mod db;
mod employees;
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    db::init();
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| {
        App::new()
            .configure(employees::init_routes)
    });
    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l).unwrap()
    } else {
        server.bind(format!("{}:{}", env::var("HOST").unwrap(), env::var("PORT").unwrap()).as_str()).unwrap()
    };
    server.run().await
}
