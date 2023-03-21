use diesel::Queryable;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Motds {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub mode: Option<String>,
    pub game_mode: Option<String>,
    pub max_players: Option<i32>,
    pub ret_msg: Option<String>,
    pub title: Option<String>,
    pub start_date_time: Option<String>
}
