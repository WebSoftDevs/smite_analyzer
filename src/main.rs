#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc, clippy::missing_errors_doc)]
pub mod api_request;
pub mod motd;
pub mod motd_mode;

use crate::api_request::SmiteApiClient;

use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let smite_dev_key =
        std::env::var("SMITE_DEV_KEY").expect("SMITE_DEV_KEY not present in env variables");

    let smite_dev_id =
        std::env::var("SMITE_DEV_ID").expect("SMITE_DEV_ID not present in env variables");

    let mut client = SmiteApiClient::new(smite_dev_key, smite_dev_id);

    let motds = client.get_motd().await.unwrap();

    dbg!(motds);
}
