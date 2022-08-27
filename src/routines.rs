use reqwest::{Client, Error};

use crate::models::*;

// 1 r&eal = 100000
pub fn goodies(analyze: Vec<ApiResponse>, price: u64) -> Vec<ItemsBasic> {
    let mut goodies: Vec<ItemsBasic> = Vec::new();
    for tmp in analyze {
        for item in tmp.items {
            let bitem = item.item_basic;
            if bitem.stock > 0
                && bitem.item_rating.rating_star > 4.7
                && bitem.item_rating.rcount_with_context > 20
                && bitem.item_rating.rcount_with_image > 10
                && bitem.sold > 15
                && bitem.price < price
                && bitem.liked_count > 50
                && bitem.shop_location != Some("China Continental".to_string())
                && bitem.price_before_discount >= bitem.price
            {
                goodies.push(bitem)
            }
        }
    }
    goodies
}

pub async fn get_all_pages(bot: Client, produto: &str) -> Result<Vec<ApiResponse>, Error> {
    let mut pages: Vec<ApiResponse> = Vec::new();
    let mut counter: usize = 0;
    loop {
        let pesquisa = format!("https://shopee.com.br/api/v4/search/search_items?by=relevancy&keyword={}&limit=60&newest={}&order=desc&page_type=search&scenario=PAGE_GLOBAL_SEARCH&version=2",produto,counter);

        let res = bot.get(&pesquisa).send().await?;
        match res.status() {
            reqwest::StatusCode::OK => {
                let json = res.json::<ApiResponse>().await?;

                if json.nomore {
                    break;
                } else {
                    counter += 60;
                }
                pages.push(json);
            }

            reqwest::StatusCode::UNAUTHORIZED => {
                println!("should not be here");
                break;
            }

            rest => {
                println!("{:?}", rest);
            }
        }
    }
    Ok(pages)
}

pub fn pretty_print(item: &ItemsBasic) {
    println!("-----------------------------");
    println!("nome:{}", item.name);
    println!("preco:{}", item.price / 1000);
    println!("avaliacoes:{:?}", item.item_rating);
    println!("vendeu:{}", item.sold);
    println!("preco_original:{}", item.price_before_discount / 1000);
    println!("localizacao:{:?}", item.shop_location);
    println!("desconto:{}%", item.show_discount);
    println!("likes:{}", item.liked_count);
}
