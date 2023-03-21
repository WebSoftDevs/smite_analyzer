#[macro_use] extern crate diesel;
use diesel::backend::Backend;
use serde::{Deserialize, Serialize};
use diesel::sql_types::VarChar;
use diesel::deserialize::{FromSql, self};
use diesel::serialize::{ToSql, Output, self};

#[derive(Debug, Deserialize, Serialize, FromSqlRow)]
#[serde(from = "String")]
pub enum MotdMode {
    AllOutAssault,
    InfiniteAssault,
    Omnipotence,
    OmnipotentDraft,
    TillDeath,
    GrabBag,
    CooldownsArena,
    AllInArena,
    Other(String),
}

impl From<String> for MotdMode {
    fn from(value: String) -> Self {
        dbg!(&value);
        match value.as_str() {
            "All Out Assault 2.0" => MotdMode::AllOutAssault,
            "Infinite Assault" => MotdMode::InfiniteAssault,
            "Omnipotence" => MotdMode::Omnipotence,
            "Omnipotent Draft" => MotdMode::OmnipotentDraft,
            "Till Death" => MotdMode::TillDeath,
            "Grab Bag 2.0" => MotdMode::GrabBag,
            "Cooldowns Runneth Over" => MotdMode::CooldownsArena,
            "All In Arena" => MotdMode::AllInArena,
            unknown => MotdMode::Other(unknown.to_string()),
        }
    }
}


impl<DB> ToSql<VarChar, DB> for MotdMode
where
    DB: Backend,
    String: ToSql<VarChar, DB>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, DB>) -> serialize::Result {
        match *self{
            MotdMode::AllInArena => "All In Arena",
            MotdMode::InfiniteAssault => "Infinite Assault",
            MotdMode::Omnipotence => "Omnipotence",
            MotdMode::OmnipotentDraft => "Omnipotent Draft",
            MotdMode::TillDeath => "Till Death",
            MotdMode::GrabBag => "Grab Bag 2.0",
            MotdMode::CooldownsArena => "Cooldowns Runneth Over",
            MotdMode::AllOutAssault => "All Out Assault 2.0",
            _ => "test"
        }.to_string().to_sql(out)
    }
}

impl<DB> FromSql<VarChar, DB> for MotdMode
where
    DB: Backend,
    String: FromSql<VarChar, DB>,
{
    fn from_sql(bytes: DB::RawValue<'_>) -> deserialize::Result<Self> {
        match String::from_sql(bytes)? {
            Some("All Out Assault 2.0") => Ok(MotdMode::AllOutAssault),
            Some("Infinite Assault") => Ok(MotdMode::InfiniteAssault),
            Some("Omnipotence") => Ok(MotdMode::Omnipotence),
            Some("Omnipotent Draft") => Ok(MotdMode::OmnipotentDraft),
            Some("Till Death") => Ok(MotdMode::TillDeath),
            Some("Grab Bag 2.0") => Ok(MotdMode::GrabBag),
            Some("Cooldowns Runneth Over") => Ok(MotdMode::CooldownsArena),
            Some("All In Arena") => Ok(MotdMode::AllInArena),
        }
    }
}
