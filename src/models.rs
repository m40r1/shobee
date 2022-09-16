use serde::{Deserialize, Serialize};

///the complete json response
#[derive(Deserialize, Serialize, Debug)]
pub struct ApiResponse {
    pub error: Option<u32>,
    pub error_msg: Option<String>,
    pub items: Vec<Item>,
    pub nomore: Option<bool>,
    pub total_count: Option<u32>,
    pub has_more: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ApiResponse2 {
    pub error: Option<u32>,
    pub error_msg: Option<String>,
    pub data: SimpleShop,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ApiResponse3 {
    pub error: Option<u32>,
    pub error_msg: Option<String>,
    pub data: Shop,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SimpleShop {
    pub account: Account,
}

// #[derive(Serialize, Deserialize, Debug)]
// pub struct Data {
//     // voucher_list: Option<VoucherList>,
//     // shop_categories: Vec<ShopCategory>,
//     items: Vec<Item>,
// pub has_more: Option<bool>,
// }

// #[derive(Deserialize, Serialize, Debug)]
// pub struct VoucherList {
//     //check if its a vec
//     pub voucher: Voucher,
// }

#[derive(Deserialize, Serialize, Debug)]
pub struct Shop {
    // treat it as a  vec of item_basic
    pub items: Option<Vec<Item>>,
    pub last_active_time: Option<u64>,
    pub country: Option<String>,
    pub is_shopee_verified: bool,
    pub is_official_shop: bool,
    pub chat_disabled: bool,
    pub rating_normal: u64,
    pub rating_bad: u64,
    pub rating_good: u64,
    pub description: Option<String>,
    pub cancellation_rate: u64,
    //Item rating kindof
    // pub buyer_rating: u16,
    pub vacation: bool,
    pub rating_star: Option<f64>,
    pub userid: u64,
    pub shopid: u64,
    pub name: Option<String>,
    pub item_count: u32,
    pub follower_count: u32,
    pub response_rate: u8,
    pub response_time: Option<u64>,
    pub account: Account,
    pub has_shopee_flash_sale: bool,
    // pub has_in_shopee_flash_sale: bool,
    pub has_brand_sale: bool,
    pub is_preferred_plus_seller: bool,
    pub address: Option<String>,
    // pub priority_flash_sale_group: SaleGroup,
}

// #[derive(Deserialize, Serialize, Debug)]
// pub struct SaleGroup {
//     pub start_time: u64,
//     pub end_time: u64,
//     pub is_ongoing: bool,
//     pub sessions: Item,
// }

#[derive(Deserialize, Serialize, Debug)]
pub struct Account {
    pub username: String,
    pub following_count: Option<u32>,
    pub is_seller: Option<bool>,
    pub phone_verified: Option<bool>,
    pub email_verified: Option<bool>,
    pub total_avg_star: Option<f64>,
    pub hide_likes: Option<u8>,
}
/// wraper for item basic in collected json
#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    pub item_basic: ItemsBasic,
}

///data for each product
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ItemsBasic {
    pub itemid: u64,
    pub shopid: u64,
    pub catid: u64,
    pub name: String,
    pub currency: Option<String>,
    pub stock: u64,
    pub sold: u32,
    pub liked_count: u64,
    pub item_status: Option<String>,
    pub price: u64,
    pub price_before_discount: u64,
    pub show_discount: u8,
    pub item_rating: ItemRating,
    pub shopee_verified: Option<bool>,
    pub is_official_shop: Option<bool>,
    pub shop_location: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ItemRating {
    pub rating_star: f64,
    // Vec[total stars,1 stars,2,3,4,5]
    pub rating_count: RatingCount,
    pub rcount_with_context: u32,
    pub rcount_with_image: u32,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct RatingCount(u32, u32, u32, u32, u32, u32);

// #[derive(Serialize, Deserialize, Debug)]
// pub struct Voucher {
//     //TODO Voucher fields
// }

//TODO Impl for headers
// type Header = HeaderMap;
// impl Header {}

// #[derive(Serialize, Deserialize, Debug)]
// pub struct ShopCategory {
//     //TODO map shop_categoty_id
// }
