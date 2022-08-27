use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ApiResponse {
    pub items: Vec<Item>,
    pub nomore: bool,
}

#[derive(Deserialize, Debug)]
pub struct Item{
    pub item_basic: ItemsBasic,
}

#[derive(Deserialize, Debug)]
pub struct ItemsBasic {
    pub itemid: u64,
    pub shopid: u64,
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
    pub shopee_verified: bool,
    pub is_official_shop: bool,
    pub shop_location: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ItemRating {
    pub rating_star: f64,
    // Vec[total stars,1 stars,2,3,4,5]
    pub rating_count: RatingCount,
    pub rcount_with_context: u32,
    pub rcount_with_image: u32,
}

#[derive(Deserialize, Debug)]
pub struct RatingCount(u32, u32, u32, u32, u32, u32);
