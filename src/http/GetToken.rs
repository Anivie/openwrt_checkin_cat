use std::error::Error;

use reqwest::blocking::Client;

use crate::pojo::cutom_pojo::CustomAccount;
use crate::pojo::dukou_pojo::TokenResponse;
use crate::pojo::headers::token_header;

impl CustomAccount {
    pub fn post_for_token(&self, http_client: &Client) -> Result<TokenResponse, Box<dyn Error>> {

        let body = serde_json::to_string(self)?;
        let headers = token_header(&body.len());

        let response = http_client
            .post("https://dukou.to/api/token")
            .headers(headers)
            .body(body)
            .send()?;

        let result: TokenResponse = response.json()?;

        Ok(result)
    }
}