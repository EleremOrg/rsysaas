pub mod models;
pub mod routes;
pub mod utils;
pub mod versioning;
pub mod views;

pub use models::{RequestModel, ResponseModel};
pub use routes::routes;
pub use utils::auth;
pub use versioning::Version;
pub use views::{error_404, handle_items, handle_recommendations, handle_users, home};
