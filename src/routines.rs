use reqwest::Client;

use crate::models::*;

// 1 real = 100000
pub fn goodies(
    analyze: &Vec<ApiResponse>,
    price: u64,
    mut goodies: Vec<ItemsBasic>,
) -> Vec<ItemsBasic> {
    for tmp in analyze {
        for item in &tmp.items {
            let bitem = &item.item_basic;
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
                goodies.push(bitem.clone())
            }
        }
    }
    goodies
}

//  TODO redo with true async
// but its kind ok
pub async fn get_all_pages(bot: Client, produto: &str) -> Result<Vec<ApiResponse>, reqwest::Error> {
    let mut pages: Vec<ApiResponse> = Vec::new();
    let mut counter: usize = 0;
    loop {
        let pesquisa = format!("https://shopee.com.br/api/v4/search/search_items?by=relevancy&keyword={}&limit=60&newest={}&order=desc&page_type=search&scenario=PAGE_GLOBAL_SEARCH&version=2",produto,counter);

        let res = bot.get(&pesquisa).send().await?;

        let json = res.json::<ApiResponse>().await?;

        //is this what  real rust looks like?
        if match json.nomore {
            Some(i) => i,
            None => false,
        } {
            break;
        } else {
            counter += 60;
        }
        pages.push(json);
    }
    Ok(pages)
}

pub async fn get_shop_name(bot: Client, shopid: u64) -> Result<String, reqwest::Error> {
    let pesquisa = format!(
        "https://shopee.com.br/api/v4/product/get_shop_info?shopid={}",
        shopid
    );

    //TODO err handle??
    let res = bot.get(&pesquisa).send().await?;

    //TODO Err handle this
    let shop = res.json::<ApiResponse2>().await?;

    Ok(shop.data.account.username)
}

pub async fn get_shop_details(bot: Client, username: String) -> Result<Shop, reqwest::Error> {
    //TODO Need to get shop_info first
    // use tokio::fs;

    let pesquisa = format!(
        "https://shopee.com.br/api/v4/shop/get_shop_detail?sort_sold_out=0&username={}",
        username
    );

    let res = bot.get(&pesquisa).send().await?;
    // let buf = res.text().await?;
    // fs::write("get_shop_details.json", buf).await.unwrap();
    let shop = res.json::<ApiResponse3>().await?;

    Ok(shop.data)
}

pub fn is_good_shop(shop: &Shop) -> bool {
    if shop.item_count > 0
        && !shop.chat_disabled
        && shop.rating_good > 300
        && shop.rating_bad < 50
        && shop.rating_normal > 10
        // &&  shop.is_official_shop
        &&  shop.is_shopee_verified
        && shop.cancellation_rate < 5
        // && shop.buyer_rating > 5
        && !shop.vacation
        && shop.follower_count > 200
        && shop.response_rate > 60
        && match shop.account.email_verified {
            Some(i) => i,
            None => false
        }
        &&  match shop.account.phone_verified {
            Some(i) => i,
            None => false
        }
        && match shop.account.is_seller {
            Some(i) => i,
            None => false
        }
        // && match shop.account.hide_likes  {
        //     Some(i) => i,
        //     None => false
        // }
        && match shop.account.total_avg_star {
                        Some(i) => i > 4.5,
            None => false
        }
        && match shop.rating_star {
        Some(i) => i > 4.7,
        None => false
        }
    {
        true
    } else {
        false
    }
}

// pub async fn get_shop_voucher(bot:Client,shopid: u64) -> Voucher {

// }

// pub async fn get_shop_category(bot: Client, shopid: u64) -> Vec<ShopCategory> {}

// pub fn pretty_print(item: &ItemsBasic) {
//     println!("-----------------------------");
//     println!("nome:{}", item.name);
//     println!("preco:{}", item.price / 1000);
//     println!("avaliacoes:{:?}", item.item_rating);
//     println!("vendeu:{}", item.sold);
//     println!("preco_original:{}", item.price_before_discount / 1000);
//     println!("localizacao:{:?}", item.shop_location);
//     println!("desconto:{}%", item.show_discount);
//     println!("likes:{}", item.liked_count);
// }
