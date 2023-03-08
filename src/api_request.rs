use anyhow;
use reqwest::Url;
use thiserror::Error;
use url::ParseError;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("Couldn't add '{1}' to url")]
    UrlParseError(#[source] ParseError, String),
}

pub struct SmiteApiClient {
    base_url: Url,
    dev_key: String,
    dev_id: String,
}

impl SmiteApiClient {
    //TODO
    pub fn get(&self, _endpoint: String) {}

    pub fn new(dev_key: String, dev_id: String) -> SmiteApiClient {
        let base_url = Url::parse("https://api.smitegame.com/smiteapi.svc/").unwrap();

        SmiteApiClient {
            base_url,
            dev_key,
            dev_id,
        }
    }

    pub async fn open_session(&self) {
        let url: Url = self
            .base_url
            .join(
                format!(
                    "{}/{}/{}/{}",
                    "createsessionJson",
                    &self.dev_id,
                    self.create_signature("createsession".to_string()),
                    Self::timestamp()
                )
                .as_str(),
            )
            .expect("BAR");

        dbg!(&url.as_str());

        let response = reqwest::get(url)
            .await
            .expect("No response")
            .text()
            .await
            .unwrap();
        dbg!(response);
    }

    pub fn create_request(&self, method_name: String) -> Result<Url, anyhow::Error> {
        let signature = self.create_signature(method_name.to_owned());
        let method = format!("{}Json", method_name);
        let timestamp = Self::timestamp();
        Ok(self
            .base_url
            .join(
                &format!(
                    "{}/{}/{}/{}/{}",
                    &method, &self.dev_id, &signature, &self.dev_key, &timestamp
                )
                .to_string(),
            )
            .map_err(|err| ClientError::UrlParseError(err, method_name))?)
    }

    pub fn create_signature(&self, method_name: String) -> String {
        dbg!(Self::timestamp());
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

        format!("{:x}", hash)
    }

    pub fn timestamp() -> String {
        let now = chrono::Utc::now();
        now.format("%Y%m%d%H%M%S").to_string()
    }
}
