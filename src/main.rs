mod config;
mod upload;

use actix_web::{web::Data, App, HttpServer};

use crate::config::Config;

#[actix_rt::main]
async fn main() {
    let config = Config::load().expect("could not load config");
    let data = Data::new(config);

    HttpServer::new(move || App::new().app_data(data.clone()).service(upload::upload))
        .bind("0.0.0.0:8080")
        .expect("could not bind port")
        .run()
        .await
        .expect("error while running server")
}
