mod rc;

use anyhow::Result;
use rc::{parse_from_str, ZulipRuntimeConfig};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "result")]
pub enum SendMessageResponse {
    #[serde(rename = "success")]
    Success { id: i64, msg: String },
    #[serde(rename = "error")]
    Error {
        code: String,
        msg: String,
        stream: Option<String>,
    },
}

#[derive(Serialize, Debug)]
#[serde(tag = "type")]
pub enum Message {
    #[serde(rename = "stream")]
    Stream {
        to: String,
        topic: String,
        content: String,
    },
    #[serde(rename = "private")]
    Private { to: String, content: String },
}

pub struct Client {
    rc: ZulipRuntimeConfig,
}

impl Client {
    pub fn new(rc_str: &str) -> Result<Self> {
        let rc = parse_from_str(rc_str)?;
        Ok(Client { rc })
    }
    pub fn parse(rc_str: &str) -> Result<Self> {
        let rc = parse_from_str(rc_str)?;
        Ok(Client { rc })
    }
    pub async fn send_message(&self, msg: Message) -> Result<SendMessageResponse> {
        let client = reqwest::Client::new();
        let result = client
            .post(&format!("{}/api/v1/messages", &self.rc.api.site))
            .basic_auth(&self.rc.api.email, Some(&self.rc.api.key))
            .header("application", "x-www-form-urlencoded")
            .form(&msg)
            .send()
            .await?;
        let resp: SendMessageResponse = result.json().await?;
        Ok(resp)
    }
}
