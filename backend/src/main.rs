#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc, clippy::missing_errors_doc)]
pub mod api_request;
pub mod db;
pub mod motd;
pub mod motd_mode;
pub mod schema;

use crate::{api_request::SmiteApiClient, motd::Motd};

use chrono::DateTime;
use diesel::{PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};
use dotenv::dotenv;
use schema::motds;

#[tokio::main]
async fn main() {
    dotenv().ok();
    //
    let smite_dev_key =
        std::env::var("SMITE_DEV_KEY").expect("SMITE_DEV_KEY not present in env variables");

    let smite_dev_id =
        std::env::var("SMITE_DEV_ID").expect("SMITE_DEV_ID not present in env variables");

    let mut client = SmiteApiClient::new(smite_dev_key, smite_dev_id);
    println!("Hi");

    use self::schema::motds::dsl::motds;
    let mut connection = db::open_connection();
    let results: Vec<Motd> = motds
        .limit(5)
        .select(Motd::as_select())
        .load(&mut connection)
        .unwrap();

    println!("Displaying {} motds", results.len());
    for result in results {
        dbg!(result);
    }
}

async fn insert_motd(client: &mut SmiteApiClient, conn: &mut PgConnection) {
    let res = client.get_motd().await.unwrap();
    diesel::insert_into(motds::table)
        .values(&res)
        .execute(conn)
        .unwrap();
}
