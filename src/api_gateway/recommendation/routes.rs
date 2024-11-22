use stefn::{APIState, AppResult, ErrorMessage};

use axum::{
    extract::{Path, Query},
    routing::get,
    Extension, Json, Router,
};
use serde::{Deserialize, Serialize};
use utoipa::{self, IntoParams, OpenApi, ToSchema};

use crate::{
    api_gateway::auth::JWTUser,
    entities::products::Category,
    rec_service::{Recommendation, RecommendationClient},
};

#[derive(OpenApi)]
#[openapi(
    paths(get_recommendation),
    components(schemas(Recommendation),
    responses(Recommendation)),
    security(("token_jwt" = []))
)]
pub struct ApiDoc;

pub fn routes(state: APIState) -> Router<APIState> {
    Router::new()
        .route("/:product", get(get_recommendation))
        .with_state(state)
}

#[derive(Debug, Serialize, Deserialize, IntoParams, ToSchema)]
struct RecommendationQuery {
    prod_id: Option<String>,
    user_id: Option<String>,
    quantity: u8,
}

#[utoipa::path(
    get,
    path = "/movies",
    params(RecommendationQuery),
    responses(
        (status = 200, body = Vec<Recommendation>, description = "Recommendations for a client"),
        (status = "4XX", body = ErrorMessage, description = "Opusi daisy, you messed up"),
        (status = "5XX", body = ErrorMessage, description = "Opusi daisy, we messed up, sorry"),
    )
)]
async fn get_recommendation(
    Extension(user): JWTUser,
    Path((_version, category)): Path<(String, Category)>,
    Query(query): Query<RecommendationQuery>,
) -> AppResult<Vec<Recommendation>> {
    let mut rec = RecommendationClient::new(category, user.id);
    if let Some(target_id) = query.user_id {
        rec = rec.set_target_id(target_id);
    }
    if let Some(prod_id) = query.prod_id {
        rec = rec.set_product_id(prod_id);
    }
    rec.recommend().await.map(Json)
}
