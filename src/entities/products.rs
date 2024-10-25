use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::ser::to_vec;
use std::fmt::Debug;
use utoipa::{self, ToSchema};

#[derive(Debug, Serialize)]
struct ProductEventPayload<'a, T> {
    customer_id: i64,
    client_id: i64,
    product: &'a Product<T>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Product<T> {
    id: String,
    price: f64,
    currency: String,
    image: String,
    url: String,
    description: String,
    specs: T,
}

impl<T> Product<T> {
    fn to_event(&self, customer_id: i64, client_id: i64) -> ProductEventPayload<T> {
        ProductEventPayload {
            customer_id,
            client_id,
            product: self,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(tag = "category", content = "products")]
pub enum ProductCategory {
    Clothing(Vec<Product<ClothingProduct>>),
    SportsAndOutdoors(Vec<Product<SportsAndOutdoorsProduct>>),
    BooksAndMedia(Vec<Product<BooksAndMediaProduct>>),
}

impl ProductCategory {
    pub fn to_events(&self, customer_id: i64, client_id: i64) -> Vec<Vec<u8>> {
        match self {
            Self::BooksAndMedia(v) => Self::products_to_events(v, customer_id, client_id),
            Self::Clothing(v) => Self::products_to_events(v, customer_id, client_id),
            Self::SportsAndOutdoors(v) => Self::products_to_events(v, customer_id, client_id),
        }
    }

    fn products_to_events<T: Serialize + DeserializeOwned>(
        products: &Vec<Product<T>>,
        customer_id: i64,
        client_id: i64,
    ) -> Vec<Vec<u8>> {
        products
            .into_iter()
            .filter_map(|p| to_vec(&p.to_event(customer_id, client_id)).ok())
            .collect()
    }
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
    Movies,
    TVShows,
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
