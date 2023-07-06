pub mod requests;
pub mod responses;
pub mod routes;
pub mod views;

pub use requests::{PathRequest, QueryRequest, RecommendationQueryRequest};
pub use responses::{max_limit, non_auth, not_found, our_fault, success};
pub use views::{
    delete_entities, error_404, get_entities, get_recommendations, home, list_entities,
    patch_entities, post_entities, put_entities, sse_handler, ws_handler,
};
