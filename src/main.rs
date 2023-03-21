#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc, clippy::missing_errors_doc)]

pub mod api_request;
pub mod motd;
pub mod motd_mode;
pub mod handlers;
pub mod models;
pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

extern crate diesel;
// use crate::api_request::SmiteApiClient;

use actix_web::{App, HttpServer, middleware, web};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;


mod schema;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    // we do want to fire below code right now, let's commend it
    // let smite_dev_key =
    // std::env::var("SMITE_DEV_KEY").expect("SMITE_DEV_KEY not present in env variables");
    // let smite_dev_id =
    //     std::env::var("SMITE_DEV_ID").expect("SMITE_DEV_ID not present in env variables");
    // let mut client = SmiteApiClient::new(smite_dev_key, smite_dev_id);
    // let motds = client.get_motd().await.unwrap();
    // dbg!(motds);


    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: DbPool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move|| {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(handlers::motds)
            .wrap(middleware::Logger::default())
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
