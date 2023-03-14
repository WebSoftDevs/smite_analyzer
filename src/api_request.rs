use anyhow;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;
use url::ParseError;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("Couldn't add '{1}' to url")]
    UrlParseError(#[source] ParseError, String),
    #[error("Couldn't get a session from smite api.")]
    SessionResponseError(reqwest::Error),
    #[error("Couldn't get a request from smite api.")]
    RequestResponseError(reqwest::Error),
    #[error("Couldn't parse session {0}")]
    RequestParseError(serde_json::Error, String),
    #[error("Invalid session status: {0}")]
    InvalidSession(String),
}

pub struct SmiteApiClient {
    base_url: Url,
    dev_key: String,
    dev_id: String,
    session: Option<Session>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    ret_msg: String,
    session_id: String,
    timestamp: String,
}

impl SmiteApiClient {
    pub async fn get(&mut self, endpoint: String) -> Result<Value, ClientError> {
        let session: &Session = if let Some(api_session) = &self.session {
            api_session
        } else {
            self.session = Some(self.open_session().await?);

            self.session.as_ref().unwrap()
        };

        let url_str = format!(
            "{}json/{}/{}/{}/{}",
            endpoint.clone(),
            &self.dev_id,
            self.create_signature(&endpoint),
            session.session_id,
            Self::timestamp()
        );
        //TODO: Add auto renewing session after it expires

        let url: Url = self
            .base_url
            .join(&url_str)
            .map_err(|err| ClientError::UrlParseError(err, url_str.clone()))?;

        let response = reqwest::get(url)
            .await
            .map_err(ClientError::RequestResponseError)?
            .text()
            .await
            .map_err(ClientError::RequestResponseError)?;

        let parsed_response: Value = serde_json::from_str(&response)
            .map_err(|err| ClientError::RequestParseError(err, response))?;

        Ok(parsed_response)
    }

    #[must_use]
    pub fn new(dev_key: String, dev_id: String) -> SmiteApiClient {
        let base_url = Url::parse("https://api.smitegame.com/smiteapi.svc/").unwrap();

        SmiteApiClient {
            base_url,
            dev_key,
            dev_id,
            session: None,
        }
    }

    pub async fn open_session(&self) -> Result<Session, ClientError> {
        let url_str = format!(
            "{}/{}/{}/{}",
            "createsessionJson",
            &self.dev_id,
            self.create_signature("createsession"),
            Self::timestamp()
        );

        let url: Url = self
            .base_url
            .join(&url_str)
            .map_err(|err| ClientError::UrlParseError(err, url_str.clone()))?;

        let response = reqwest::get(url)
            .await
            .map_err(ClientError::SessionResponseError)?
            .text()
            .await
            .map_err(ClientError::SessionResponseError)?;

        let session: Session = serde_json::from_str(&response)
            .map_err(|err| ClientError::RequestParseError(err, response))?;

        if session.ret_msg == "Approved" {
            Ok(session)
        } else {
            Err(ClientError::InvalidSession(session.ret_msg))
        }
    }

    pub fn create_request(&self, method_name: String) -> Result<Url, anyhow::Error> {
        let signature = self.create_signature(&method_name);
        let method = format!("{method_name}Json");
        let timestamp = Self::timestamp();
        Ok(self
            .base_url
            .join(
                &format!(
                    "{}/{}/{}/{}/{}",
                    &method, &self.dev_id, &signature, &self.dev_key, &timestamp
                )
            )
            .map_err(|err| ClientError::UrlParseError(err, method_name))?)
    }

    #[must_use]
    pub fn create_signature(&self, method_name: &str) -> String {
        let hash = md5::compute(
            format!(
                "{}{}{}{}",
                self.dev_id,
                method_name,
                self.dev_key,
                Self::timestamp()
            )
            .as_bytes(),
        );

        format!("{hash:x}")
    }

    #[must_use]
    pub fn timestamp() -> String {
        let now = chrono::Utc::now();
        now.format("%Y%m%d%H%M%S").to_string()
    }
}
