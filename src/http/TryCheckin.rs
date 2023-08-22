use std::error::Error;
use std::fs::File;
use std::io::Read;

use log::{error, info};
use reqwest::blocking::Client;

use crate::pojo::cutom_pojo::{CustomToken, SendKey};
use crate::pojo::dukou_pojo::CheckinResponse;
use crate::pojo::headers::checkin_header;

pub trait CheckIn {
    fn try_checkin(&mut self, http_client: &Client);
}

impl CheckIn for Vec<CustomToken> {
    fn try_checkin(&mut self, http_client: &Client) {
        for token in self.clone() {
            let result = checkin(http_client, &token.token).unwrap();
            if result.ret != 1 {
                error!("{:?} checkin failed", result);
                send_to_wechat(http_client, &result);
                self.retain(|x| !x.email.eq(&token.email));
            } else {
                info!("{} checkin success", token.email);
                send_to_wechat(http_client, &result);
            }
        }
    }
}

fn send_to_wechat(http_client: &Client, result: &CheckinResponse) {
    for x in get_sendkey() {
        http_client
            .get(format!("https://sctapi.ftqq.com/{}.send?title={:?}", x.key, result))
            .send()
            .unwrap();
    }
}
fn get_sendkey() -> Vec<SendKey> {
    let mut file = File::open("config/SendKey.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    serde_json::from_str(&contents).unwrap()
}

fn checkin(http_client: &Client, token: &str) -> Result<CheckinResponse, Box<dyn Error>> {
    let headers = checkin_header(token);

    let response = http_client
        .get("https://dukou.to/api/user/checkin")
        .headers(headers)
        .send()
        .unwrap();

    let result: CheckinResponse = response.json().unwrap();

    Ok(result)
}