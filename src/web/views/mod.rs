pub mod api;
pub mod regular;
pub mod sse;
pub mod ws;

pub use api::{
    delete_entities, get_entities, get_recommendations, list_entities, patch_entities,
    post_entities, put_entities,
};
pub use regular::{error_404, home};
pub use sse::sse_handler;
pub use ws::ws_handler;
