mod dto;
mod model;
mod route;
use crate::route::init_routes;
use actix_web::http::ContentEncoding;
use actix_web::{middleware, web, App, HttpServer};
use mongodb::{options::ClientOptions, Client};
use std::env;
use std::sync::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let mongo_url = env::var("MONGO_URI").unwrap();
    println!("{}", mongo_url);
    let mut client_options = ClientOptions::parse(&mongo_url).await.unwrap();
    client_options.app_name = Some("URL-Shortener".to_string());
    let client = web::Data::new(Mutex::new(Client::with_options(client_options).unwrap()));

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Compress::new(ContentEncoding::Br))
            .wrap(middleware::Logger::default())
            .app_data(client.clone())
            .service(web::scope("/l").configure(init_routes))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
