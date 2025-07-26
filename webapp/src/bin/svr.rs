#[path = "../mod.rs"]
mod wa;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use routers::app_config;
use std::env;
use wa::{errors, handlers, models, routers};

use tera::Tera;

#[actix_web::main]
pub async fn main() -> std::io::Result<()>
{
    dotenv().ok();
    let host_port = env::var("HOST_PORT").expect("HOST:PORT address is not set in .env file");
    println!("Listening on: {}", &host_port);

    HttpServer::new(move ||
    {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static/**/*")).unwrap();
        App::new().app_data(web::Data::new(tera)).configure(app_config)
    })
    .bind(&host_port)?
    .run()
    .await
}

// #[actix_web::main]
// pub async fn main() -> std::io::Result<()> {
//     dotenv().ok();
//     let host_port = env::var("HOST_PORT").expect("HOST:PORT address is not set in .env file");
//     println!("Listening on: {}", &host_port);

//     let tera = Tera::new("static/*.html").map_err(|e| {
//         eprintln!("Failed to initialize Tera: {}", e);
//         std::io::Error::new(std::io::ErrorKind::Other, "Tera initialization failed")
//     })?;
//     HttpServer::new(move || {
//         App::new().app_data(web::Data::new(tera.clone())).configure(app_config)
//     })
//     .bind(&host_port)?
//     .run()
//     .await
// }