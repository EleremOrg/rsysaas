use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use utoipa::{self, ToSchema};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Product<T> {
    pub id: String,
    price: f64,
    currency: String,
    image: String,
    url: String,
    description: String,
    specs: T,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ProductUpdate<T> {
    id: String,
    values: T,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(tag = "target", content = "products")]
pub enum ProductUpdateCategory {
    Clothing(Vec<ProductUpdate<ClothingProduct>>),
    SportsAndOutdoors(Vec<ProductUpdate<SportsAndOutdoorsProduct>>),
    BooksAndMedia(Vec<ProductUpdate<BooksAndMediaProduct>>),
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(tag = "target", content = "products")]
pub enum ProductCategory {
    Clothing(Vec<Product<ClothingProduct>>),
    SportsAndOutdoors(Vec<Product<SportsAndOutdoorsProduct>>),
    BooksAndMedia(Vec<Product<BooksAndMediaProduct>>),
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ClothingProduct {
    pub category: ClothingCategory,
    pub gender: ClothingGender,
    pub size: Option<String>,
    pub material: Option<String>,
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
    pub category: BooksAndMediaCategory,
    pub title: String,
    pub author: Option<String>,
    pub format: Option<String>,
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
    pub category: SportsAndOutdoorsCategory,
    pub name: String,
    pub type_: Option<String>,
    pub material: Option<String>,
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
