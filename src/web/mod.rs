pub mod requests;
pub mod responses;
pub mod routes;
pub mod views;

pub use requests::RequestModel;
pub use responses::{max_limit, non_auth, not_found, our_fault, success};
pub use views::{
    delete_items, delete_users, error_404, get_items, get_recommendations, get_users, home,
    list_items, list_users, patch_items, patch_users, post_items, post_users, put_items, put_users,
    sse_handler, ws_handler,
};
