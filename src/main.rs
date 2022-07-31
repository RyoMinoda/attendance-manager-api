#[macro_use]
extern crate diesel;

use actix_web::{HttpServer, App, web::{Data}};
use std::{io::Result, env};

mod routes;
mod configs;
mod utils;
mod models;
mod schema;
mod repos;

use routes::group;
use configs::postgres;

#[actix_web::main]
async fn main() -> Result<()> {
    env::set_var("RUST_BACKTRACE", "1");
    let pool = postgres::get_pool();
    print!("pool is got");
    HttpServer::new(move|| {
        App::new()
            .app_data(Data::new(pool.clone()))
            .configure(group::config_groups)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}