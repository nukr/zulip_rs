mod client;
pub mod message;
pub mod rc;

pub use client::Client;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(tag = "result")]
#[serde(rename_all = "camelCase")]
pub enum CommonMutateResponse {
    Success,
    Error(CommonMessageError),
}

#[derive(Deserialize, Debug)]
pub struct CommonMessageError {
    code: String,
    msg: String,
}
