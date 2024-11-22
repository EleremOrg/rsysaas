use chrono::NaiveDateTime;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::ser::to_vec;
use std::fmt::Debug;
use utoipa::{self, ToSchema};

macro_rules! define_products {
    (
        $(
            $variant:ident => $data_type:ty,
        )*
    ) => {
        // Generate the Category enum
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "kebab-case")]
        pub enum Category {
            $(
                $variant,
            )*
        }

        // Generate the ProductPayload enum
        #[derive(Debug, Serialize, Deserialize, ToSchema)]
        #[serde(tag = "category", content = "products")]
        pub enum ProductPayload {
            $(
                $variant(Vec<Product<$data_type>>),
            )*
        }

        impl ProductPayload {
            // Create a helper to get the category
            pub fn category(&self) -> Category {
                match self {
                    $(
                        Self::$variant(_) => Category::$variant,
                    )*
                }
            }

            pub fn to_events(&self, customer_id: i64, client_id: i64) -> Vec<Vec<u8>> {
                match self {
                    $(
                        Self::$variant(v) => products_to_events(v, customer_id, client_id),
                    )*
                }
            }

            // Create a helper to deserialize dynamically based on Category
            // pub fn from_category_and_data(category: Category, data: serde_json::Value) -> Result<Self, serde_json::Error> {
            //     match category {
            //         $(
            //             Category::$variant => {
            //                 let payload: $data_type = serde_json::from_value(data)?;
            //                 Ok(ProductPayload::$variant(payload))
            //             }
            //         )*
            //     }
            // }
        }
    };
}

define_products! {
    Clothing => ClothingProduct,
    SportsAndOutdoors => SportsAndOutdoorsProduct,
    Movies => Option<String>,
}

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

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ClothingProduct {
    pub category: ClothingCategory,
    pub gender: ClothingGender,
    pub size: Option<String>,
    description: String,
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
    id: String,
    product_id: String,
    date: NaiveDateTime,
    quantity: u64,
    price: f64,
    currency: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Refund {
    id: String,
    order_id: String,
    product_id: String,
    date: NaiveDateTime,
    quantity: u64,
    price: f64,
    reason: String,
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
