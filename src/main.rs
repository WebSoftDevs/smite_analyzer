#![warn(clippy::pedantic)]
pub mod api_request;

use crate::api_request::SmiteApiClient;

use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let smite_dev_key =
        std::env::var("SMITE_DEV_KEY").expect("SMITE_DEV_KEY not present in env variables");

    let smite_dev_id =
        std::env::var("SMITE_DEV_ID").expect("SMITE_DEV_ID not present in env variables");

    let client = SmiteApiClient::new(smite_dev_key, smite_dev_id);

    client.open_session().await;
}
