#![allow(non_snake_case)]

use std::fs::File;
use std::io::{Read, Write};

use reqwest::blocking::Client;
use crate::http::MakeCache::Cache;

use crate::http::TryCheckin::CheckIn;
use crate::pojo::cutom_pojo::{CustomAccount, CustomToken};

mod pojo;
mod http;

fn main() {
    env_logger::init();
    let http_client = Client::builder()
        .brotli(true)
        .gzip(true)
        .deflate(true)
        .build()
        .unwrap();

    let mut token_cache = get_token_cache().unwrap();
    let mut all_accounts = get_accounts().unwrap();

    http_client.make_token_cache(&mut all_accounts, &mut token_cache, None);
    token_cache.try_checkin(&http_client);

    if all_accounts.len() != token_cache.len() {
        let mut new_user = Vec::new();
        http_client.make_token_cache(&mut all_accounts, &mut token_cache, Some(&mut new_user));
        new_user.try_checkin(&http_client);

        cache_token(&token_cache);
    }
}

fn cache_token(token_cache: & Vec<CustomToken>) {
    let mut file = File::create("config/Token_Cache.json").unwrap();
    file.write_all(serde_json::to_string_pretty(&token_cache).unwrap().as_bytes()).unwrap();
}
/*
fn make_token_cache(
    http_client: &Client,
    all_accounts: &mut Vec<CustomAccount>,
    custom_token_list: &mut Vec<CustomToken>,
    mut new_user: Option<&mut Vec<CustomToken>>
) {
    for x in all_accounts {
        let token_list: Vec<String> = custom_token_list.iter().map(|x| x.email.to_string()).collect();

        if token_list.contains(&x.email) {
            continue;
        }

        let response = x.post_for_token(&http_client).unwrap();

        if response.ret != 1 {
            error!("Get token failed,{:?}", &response);
            continue;
        }

        info!("Get token success,{:?}", &response.msg.unwrap());
        let custom_token = CustomToken {
            email: x.email.to_string(),
            token: response.token.unwrap(),
        };

        if let Some(new_user) = &mut new_user {
            new_user.push(custom_token.clone());
        }
        custom_token_list.push(custom_token);
    }

    cache_token(&custom_token_list);
}
*/
fn get_accounts() -> Result<Vec<CustomAccount>, serde_json::Error> {
    let mut file = File::open("config/Account.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let custom_account_list: Vec<CustomAccount> = serde_json::from_str(&contents)?;
    Ok(custom_account_list)
}

fn get_token_cache() -> Result<Vec<CustomToken>, serde_json::Error> {
    let mut file = match File::open("config/Token_Cache.json") {
        Ok(file) => file,
        Err(err) => {
            if err.kind() != std::io::ErrorKind::NotFound {
                File::create("config/Token_Cache.json").expect("Failed to create cache file!");
                return Ok(Vec::new());
            } else {
                return Ok(Vec::new());
            }
        }
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    serde_json::from_str(&contents)
}

