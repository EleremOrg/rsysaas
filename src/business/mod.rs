pub mod entities;
pub mod requests;
pub mod utils;
pub mod versioning;

pub use entities::{Customer, Recommendation};
pub use requests::RecommendationRequest;
pub use utils::auth;
pub use versioning::Version;
