use matrix_sdk::reqwest::Client;
use matrix_sdk::reqwest::Error;
use matrix_sdk::reqwest::Request;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Clone, Debug, Serialize)]
pub struct Body {
    pub sid: String,
    pub client_secret: String,
    pub token: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Response {
    pub success: bool,
}

pub fn request(base: Url, body: &Body) -> Result<Request, Error> {
    let url = base
        .join("_matrix/identity/api/v1/validate/msisdn/submitToken")
        .expect("Malformed URL in msisdn submit_token");

    let data = serde_json::to_vec(body).unwrap();

    Client::new().post(url).body(data).build()
}
