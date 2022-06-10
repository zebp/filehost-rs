mod config;
mod upload;

use std::env::var;

use actix_web::{web::Data, App, HttpServer};

use crate::config::Config;

#[actix_web::main]
async fn main() {
    let config = Config::load().expect("could not load config");
    let data = Data::new(config);

    let port = var("PORT").expect("could not get port from env variable");

    HttpServer::new(move || App::new().app_data(data.clone()).service(upload::upload))
        .bind(format!("0.0.0.0:{}", port))
        .expect("could not bind port")
        .run()
        .await
        .expect("error while running server")
}
