use actix_web::{get, HttpResponse, Responder};
use diesel::{
    Insertable, PgConnection, QueryDsl, Queryable, RunQueryDsl, Selectable, SelectableHelper,
};
use serde::{Deserialize, Deserializer, Serialize};

use crate::{
    api_request::{ClientError, SmiteApiClient},
    db,
    motd_mode::MotdMode,
    schema::{self, motds},
    SMITE_API_CLIENT,
};

/**
 * Represents MOTD from the Smite API.
 */
#[derive(Queryable, Selectable, Insertable, Debug, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::motds)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(rename_all = "camelCase")]
pub struct Motd {
    pub description: String,
    pub game_mode: String,
    #[serde(deserialize_with = "string_to_i32")]
    pub max_players: Option<i32>,
    pub name: String,
    pub ret_msg: Option<String>,
    pub start_date_time: String,
    #[serde(deserialize_with = "empty_string_is_none")]
    #[serde(rename = "team1GodsCSV")]
    pub team_1_gods_csv: Option<String>,
    #[serde(deserialize_with = "empty_string_is_none")]
    #[serde(rename = "team2GodsCSV")]
    pub team_2_gods_csv: Option<String>,
    #[serde(rename = "title")]
    mode: String,
}

impl Motd {
    #[must_use]
    pub fn motd_mode(&self) -> MotdMode {
        MotdMode::from(self.mode.clone())
    }
}

fn string_to_i32<'de, D>(deserializer: D) -> Result<Option<i32>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    if s.is_empty() {
        return Ok(None);
    }

    let num: i32 = s.parse().map_err(serde::de::Error::custom)?;

    Ok(Some(num))
}

fn empty_string_is_none<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s.is_empty() {
        Ok(None)
    } else {
        Ok(Some(s))
    }
}

impl SmiteApiClient {
    pub async fn get_motd(&mut self) -> Result<Vec<Motd>, ClientError> {
        let response = self.get("getmotd".to_string()).await?;

        let motds: Vec<Motd> = serde_json::from_value(response.clone())
            .map_err(|err| ClientError::RequestParseError(err, response.to_string()))?;

        Ok(motds)
    }
}

/**
 * Bridge structure between the REST API and the database.
 */
#[derive(Serialize, Deserialize)]
pub struct RestMotdEntity {
    id: u8,
    name: String,
    description: String,
    start_date: String,
}

/**
 * Wrapper structure for the REST API response.
 */
#[derive(Serialize, Deserialize)]
pub struct RestMotdResponse {
    todays_motd: Option<RestMotdEntity>,
    tommorows_motd: Option<RestMotdEntity>,
    #[serde(default)]
    past_motds: Vec<RestMotdEntity>,
}

pub async fn update_motds_in_db(connection: &mut PgConnection) -> anyhow::Result<Vec<Motd>> {
    let motds = SMITE_API_CLIENT.lock().await.get_motd().await?;

    diesel::delete(schema::motds::table).execute(connection)?;

    diesel::insert_into(schema::motds::table)
        .values(&motds)
        .execute(connection)?;

    Ok(motds)
}

pub fn get_motds_from_db(connection: &mut PgConnection) -> anyhow::Result<Vec<Motd>> {
    let motds = motds::table
        .select(Motd::as_select())
        .load::<Motd>(connection)?;

    Ok(motds)
}

#[get("/motd/get-all")]
pub async fn get_all_motds() -> impl Responder {
    let mut connection = db::open_connection();

    let motds = {
        let motds_from_db = get_motds_from_db(&mut connection);

        let tomorrow = (chrono::Local::now() + chrono::Duration::days(1))
            .format("%_m/%d/%Y")
            .to_string()
            .trim()
            .to_string();

        if motds_from_db.as_ref().is_ok_and(|motds| {
            motds.iter().any(|motd| {
                *motd
                    .start_date_time
                    .split_whitespace()
                    .next()
                    .unwrap_or_default()
                    .to_string()
                    == tomorrow
            })
        }) {
            motds_from_db
        } else {
            update_motds_in_db(&mut connection).await
        }
    };

    match motds {
        Ok(motds) => {
            let mut motds: Vec<RestMotdEntity> = motds
                .iter()
                .enumerate()
                .map(|(id, motd)| {
                    let start_date = motd
                        .start_date_time
                        .split_whitespace()
                        .next()
                        .unwrap_or_default()
                        .to_string();

                    RestMotdEntity {
                        id: id.try_into().unwrap(),
                        name: motd.name.clone(),
                        description: motd.description.clone(),
                        start_date,
                    }
                })
                .collect();

            let response = RestMotdResponse {
                todays_motd: motds.pop(),
                tommorows_motd: motds.pop(),
                past_motds: motds,
            };

            HttpResponse::Ok().json(response)
        }
        Err(_) => HttpResponse::NotFound().json("No motds were found."),
    }
}
