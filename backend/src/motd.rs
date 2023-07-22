use actix_web::{get, HttpResponse, Responder};
use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Deserializer, Serialize};

use crate::{
    api_request::{ClientError, SmiteApiClient},
    motd_mode::MotdMode,
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

impl From<&Motd> for RestMotdEntity {
    fn from(value: &Motd) -> Self {
        Self {
            name: value.name.clone(),
            description: value.description.clone(),
            start_date: value.start_date_time.clone(),
        }
    }
}

#[get("/motd/get-all")]
pub async fn get_all_motds() -> impl Responder {
    let mut motds: Vec<RestMotdEntity> = SMITE_API_CLIENT
        .lock()
        .await
        .get_motd()
        .await
        .unwrap()
        .iter()
        .map(Into::into)
        .collect();

    let response = RestMotdResponse {
        todays_motd: motds.pop(),
        tommorows_motd: motds.pop(),
        past_motds: motds,
    };

    HttpResponse::Ok().json(response)
}
