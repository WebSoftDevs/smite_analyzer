use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
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
