#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc, clippy::missing_errors_doc)]
pub mod api_request;
pub mod db;
pub mod motd;
pub mod motd_mode;
pub mod schema;

use crate::api_request::SmiteApiClient;

use actix_web::{App, HttpServer};
use dotenv::dotenv;
use lazy_static::lazy_static;
use motd::get_all_motds;
use tokio::sync::Mutex;

lazy_static! {
    pub static ref SMITE_API_CLIENT: Mutex<SmiteApiClient> = {
        dotenv().ok();

        let smite_dev_key =
            std::env::var("SMITE_DEV_KEY").expect("SMITE_DEV_KEY not present in env variables");

        let smite_dev_id =
            std::env::var("SMITE_DEV_ID").expect("SMITE_DEV_ID not present in env variables");

        SmiteApiClient::new(smite_dev_key, smite_dev_id).into()
    };
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("REST server started.");

    HttpServer::new(|| {
        //TODO: Set explicit cors policy.
        let cors = actix_cors::Cors::permissive();

        App::new().wrap(cors).service(get_all_motds)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
