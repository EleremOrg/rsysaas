use serde::{Deserialize, Serialize};

use utoipa::{self, ToSchema};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(tag = "target", content = "products")]
pub enum ProductCategory {
    Electronics(Vec<ElectronicsProduct>),
    Clothing(Vec<ClothingProduct>),
    HomeGoods(Vec<HomeGoodsProduct>),
    PersonalCare(Vec<PersonalCareProduct>),
    HealthAndWellness(Vec<HealthAndWellnessProduct>),
    FoodAndBeverages(Vec<FoodAndBeveragesProduct>),
    Automotive(Vec<AutomotiveProduct>),
    ToysAndGames(Vec<ToysAndGamesProduct>),
    BooksAndMedia(Vec<BooksAndMediaProduct>),
    SportsAndOutdoors(Vec<SportsAndOutdoorsProduct>),
    OfficeSupplies(Vec<OfficeSuppliesProduct>),
}

impl ProductCategory {
    pub fn len(&self) -> usize {
        match self {
            ProductCategory::Electronics(products) => products.len(),
            ProductCategory::Clothing(products) => products.len(),
            ProductCategory::HomeGoods(products) => products.len(),
            ProductCategory::PersonalCare(products) => products.len(),
            ProductCategory::HealthAndWellness(products) => products.len(),
            ProductCategory::FoodAndBeverages(products) => products.len(),
            ProductCategory::Automotive(products) => products.len(),
            ProductCategory::ToysAndGames(products) => products.len(),
            ProductCategory::BooksAndMedia(products) => products.len(),
            ProductCategory::SportsAndOutdoors(products) => products.len(),
            ProductCategory::OfficeSupplies(products) => products.len(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ElectronicsProduct {
    pub brand: String,
    pub model: String,
    pub price: f64,
    pub specs: ElectronicsSpecs,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ElectronicsSpecs {
    pub screen_size: Option<String>,
    pub battery: Option<String>,
    pub camera: Option<String>,
    pub processor: Option<String>,
    pub ram: Option<String>,
    pub storage: Option<String>,
    pub color: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ClothingProduct {
    pub category: ClothingCategory,
    pub type_: ClothingType,
    pub size: Option<String>,
    pub color: Option<String>,
    pub material: Option<String>,
    pub price: f64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub enum ClothingCategory {
    Men,
    Women,
    Children,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub enum ClothingType {
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
pub struct HomeGoodsProduct {
    pub category: HomeGoodsCategory,
    pub name: String,
    pub material: Option<String>,
    pub dimensions: Option<String>,
    pub color: Option<String>,
    pub price: f64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub enum HomeGoodsCategory {
    Furniture,
    Kitchenware,
    Bedding,
    Decor,
    Storage,
    Lighting,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PersonalCareProduct {
    pub category: PersonalCareCategory,
    pub name: String,
    pub volume: Option<String>,
    pub ingredients: Option<String>,
    pub price: f64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub enum PersonalCareCategory {
    Skincare,
    Haircare,
    Makeup,
    PersonalHygiene,
    Fragrance,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct HealthAndWellnessProduct {
    pub category: HealthAndWellnessCategory,
    pub name: String,
    pub type_: Option<String>,
    pub quantity: Option<u32>,
    pub price: f64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub enum HealthAndWellnessCategory {
    VitaminsAndSupplements,
    FitnessEquipment,
    MedicalSupplies,
    HealthMonitors,
    TherapyDevices,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct FoodAndBeveragesProduct {
    pub category: FoodAndBeveragesCategory,
    pub name: String,
    pub weight: Option<String>,
    pub expiration_date: Option<String>,
    pub price: f64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub enum FoodAndBeveragesCategory {
    Groceries,
    Snacks,
    Beverages,
    FrozenFood,
    CannedFood,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AutomotiveProduct {
    pub category: AutomotiveCategory,
    pub name: String,
    pub type_: Option<String>,
    pub price: f64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub enum AutomotiveCategory {
    CarAccessories,
    CarMaintenanceProducts,
    TiresAndParts,
    Tools,
    Electronics,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ToysAndGamesProduct {
    pub category: ToysAndGamesCategory,
    pub name: String,
    pub age_range: Option<String>,
    pub material: Option<String>,
    pub price: f64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub enum ToysAndGamesCategory {
    ChildrenToys,
    BoardGames,
    Puzzles,
    EducationalToys,
    OutdoorToys,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct BooksAndMediaProduct {
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
    eBooks,
    Audiobooks,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SportsAndOutdoorsProduct {
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
pub struct OfficeSuppliesProduct {
    pub category: OfficeSuppliesCategory,
    pub name: String,
    pub type_: Option<String>,
    pub price: f64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub enum OfficeSuppliesCategory {
    Stationery,
    OfficeFurniture,
    ComputersAndAccessories,
    OfficeTools,
    OrganizationalSupplies,
}
