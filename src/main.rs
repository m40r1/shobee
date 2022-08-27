mod models;
mod routines;
use crate::models::ItemsBasic;
use reqwest::header::{HeaderMap, HeaderValue};
// use sqlx::mysql::MySqlPoolOptions;
use routines::*;
use std::{env, time::Duration};
use tokio::task;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let pesquisa = &args[1];
    let referer = format!("https://shopee.com.br/search?keyword={}", pesquisa);

    let mut headers = HeaderMap::new();

    headers.insert(
        reqwest::header::ACCEPT,
        HeaderValue::from_static("application/json"),
    );

    headers.insert(
        reqwest::header::ACCEPT_ENCODING,
        HeaderValue::from_static("gzip, deflate, br"),
    );

    headers.insert(
        reqwest::header::ACCEPT_LANGUAGE,
        HeaderValue::from_static("en-US,en;q=0.9"),
    );

    headers.insert(
        reqwest::header::USER_AGENT,
        HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/104.0.0.0 Safari/537.36")
    );

    headers.insert(
        reqwest::header::CONTENT_TYPE,
        HeaderValue::from_static("applic&mut ation/json"),
    );

    headers.insert(
        reqwest::header::REFERER,
        HeaderValue::from_str(&referer).unwrap(),
    );

    let bot = reqwest::Client::builder()
        .timeout(Duration::new(60, 0))
        .default_headers(headers)
        .build()?;

    let analyze = get_all_pages(bot, pesquisa).await?;
    let mut _goods: Vec<ItemsBasic> = Vec::new();
    task::spawn_blocking(move || {
        _goods = goodies(analyze, 500 * 100000);
        for item in _goods {
            pretty_print(&item);
        }
    })
    .await?;

    Ok(())
}
