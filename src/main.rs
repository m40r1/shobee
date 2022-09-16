mod models;
mod routines;
use crate::models::*;
// use futures::stream::TryStreamExt;
// use mongodb::bson::doc;
use mongodb::{options::ClientOptions, Client, Database};
use reqwest::header::{HeaderMap, HeaderValue};
use routines::*;
use std::{env, time::Duration};
use tokio::task;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    //needs custom error to handle mongodb
    let db = gen_db().await.unwrap();

    match env::args().len() {
        0 => {
            eprintln!("passar primeiro argumento")
        }
        3 => {
            //this is a safe unwrap
            let pesquisa = env::args().nth(1).unwrap();
            let referer = format!("https://shopee.com.br/search?keyword={}", pesquisa);
            let headers = gen_headers(&referer);
            let arg2 = env::args().nth(2).unwrap();
            let arg2 = arg2.trim();
            let mut preco = 0;
            match arg2.parse::<u64>() {
                Ok(i) => {
                    preco = i;
                }
                Err(..) => {
                    eprintln!("failed to parse preco");
                }
            }

            // im cloning clients rn
            // needs to redo async
            let bot = reqwest::Client::builder()
                .timeout(Duration::new(60, 0))
                .default_headers(headers)
                .build()?;

            let analyze = get_all_pages(bot.clone(), &pesquisa).await?;

            let mut goods: Vec<ItemsBasic> = Vec::new();

            let goods = task::spawn_blocking(move || {
                goods = goodies(&analyze, preco * 100000, goods);
                goods
            });
            let item_vec = goods.await.unwrap();
            let item_basic = db.collection::<ItemsBasic>("item_basic");
            let shop = db.collection::<Shop>("shop");

            let mut name_list: Vec<String> = Vec::new();
            for item in &item_vec {
                let name = get_shop_name(bot.clone(), item.shopid).await?;
                name_list.push(name);
            }
            item_basic.insert_many(item_vec, None).await.unwrap();
            name_list.sort();
            name_list.dedup();
            // ///TODO a lot of shit
            // /// with saving
            // /// will look into it all later
            // /// need to have some vecs
            // /// too tired rn

            let mut shop_list: Vec<Shop> = Vec::new();
            let mut good_shop: Vec<Shop> = Vec::new();
            for name in name_list {
                let shop = get_shop_details(bot.clone(), name).await?;
                shop_list.push(shop);
            }
            let shop_handle = task::spawn_blocking(move || {
                for shop in shop_list {
                    if is_good_shop(&shop) {
                        good_shop.push(shop);
                    }
                }
                good_shop
            });

            let shop_list = shop_handle.await.unwrap();
            shop.insert_many(shop_list, None).await.unwrap();
        }

        _ => {
            eprintln!("wrong number of args");
            eprintln!("just pass a search string");
            eprintln!("and a price");
        }
    }
    Ok(())
}

async fn gen_db() -> Result<Database, mongodb::error::Error> {
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    client_options.app_name = Some("bot".to_string());
    let client = Client::with_options(client_options)?;

    let db = client.database("bot");
    Ok(db)
}

fn gen_headers(referer: &str) -> HeaderMap {
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

    headers
}
