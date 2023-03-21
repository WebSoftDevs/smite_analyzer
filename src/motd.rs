use crate::{
    api_request::{ClientError, SmiteApiClient},
    motd_mode::MotdMode,
};
use diesel::Queryable;
use serde::{Deserialize, Deserializer, Serialize};

#[allow(unused_variables)]
#[derive(Debug, Serialize, Deserialize, Queryable)]
#[serde(rename_all = "camelCase")]
pub struct Motd {
    #[serde(skip)]
    pub id: i32,
    pub name: String,
    pub description: String,
    pub game_mode: String,
    pub max_players: Option<i16>,
    pub ret_msg: Option<String>,
    pub start_date_time: String,
    #[serde(deserialize_with = "empty_string_is_none")]
    #[serde(rename = "team1GodsCSV")]
    pub team_1_gods_csv: Option<String>,
    #[serde(deserialize_with = "empty_string_is_none")]
    #[serde(rename = "team2GodsCSV")]
    pub team_2_gods_csv: Option<String>,
    #[serde(rename = "title")]
    pub mode: MotdMode,
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
