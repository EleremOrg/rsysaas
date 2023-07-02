pub mod models;
pub mod requests;
pub mod responses;
pub mod utils;
pub mod versioning;

pub use models::{Customer, Recommendation};
pub use requests::RecommendationRequest;
pub use utils::auth;
pub use versioning::Version;
