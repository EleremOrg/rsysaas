use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use utoipa::{self, ToSchema};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(tag = "target", content = "products")]
pub enum ProductCategory {
    Clothing(Vec<ClothingProduct>),
    SportsAndOutdoors(Vec<SportsAndOutdoorsProduct>),
    BooksAndMedia(Vec<BooksAndMediaProduct>),
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ClothingProduct {
    pub id: String,
    pub category: ClothingCategory,
    pub gender: ClothingGender,
    pub size: Option<String>,
    pub image: String,
    pub url: String,
    pub material: Option<String>,
    pub price: f64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub enum ClothingGender {
    Men,
    Women,
    Children,
    Unisex,
    None,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub enum ClothingCategory {
    Shirt,
    Pants,
    Jacket,
    Dress,
    Skirt,
    Shorts,
    Sweater,
    Hat,
    Shoes,
    Accessories,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct BooksAndMediaProduct {
    pub id: String,
    pub category: BooksAndMediaCategory,
    pub title: String,
    pub author: Option<String>,
    pub format: Option<String>,
    pub price: f64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub enum BooksAndMediaCategory {
    Books,
    MoviesAndTVShows,
    Music,
    EBooks,
    Audiobooks,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SportsAndOutdoorsProduct {
    pub id: String,
    pub category: SportsAndOutdoorsCategory,
    pub name: String,
    pub type_: Option<String>,
    pub material: Option<String>,
    pub price: f64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub enum SportsAndOutdoorsCategory {
    SportingGoods,
    OutdoorGear,
    CampingEquipment,
    ExerciseEquipment,
    FishingGear,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Order {
    id: u64,
    product_id: u64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Refund {
    id: u64,
    order_id: u64,
    product_id: u64,
}

#[derive(Debug, Deserialize)]
pub struct Client {
    id: u64,
    name: String,
}

#[derive(Debug, Deserialize)]
pub struct Customer {
    id: u64,
    name: String,
    email: String,
    url: String,
    token: String,
    shopify_token: String,
}
