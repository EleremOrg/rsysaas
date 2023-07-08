use crate::business::{entities::CustomerInterface, requests::RecommendationRequest};
use crate::web::responses::{get_response, max_limit};
use axum::response::Response;

// TODO: fix
pub async fn throttle(
    customer: &CustomerInterface,
    rec_request: RecommendationRequest,
) -> Response {
    if can_request(customer).await {
        return get_response(&customer, rec_request).await;
    };
    return max_limit();
}

async fn can_request(_customer: &CustomerInterface) -> bool {
    true
}
