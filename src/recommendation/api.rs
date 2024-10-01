use stefn::{AppError, AppResult, AppState};

use axum::{
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use utoipa::{self, OpenApi, ToResponse, ToSchema};

#[derive(OpenApi)]
#[openapi(
    paths(get_recommendations),
    components(schemas(Recommendation),
    responses(Recommendation)),
    security(("token_jwt" = []))
)]
pub struct ApiDoc;

pub fn routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/recommendations", post(get_recommendations))
        .with_state(state)
}

#[derive(Debug, Serialize, Deserialize)]
enum RecommendationTarget {
    User,
    Product,
    Generic,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct RecommendationRequest {
    prod_id: Option<u32>,
    user_id: Option<u32>,
    number_recommendations: u8,
    target: RecommendationTarget,
}

#[derive(Debug, Serialize, Deserialize, ToResponse, ToSchema)]
struct Recommendation {
    id: u32,
    score: f32,
    url: String,
    image: String,
    title: String,
    resume: String,
}

//TODO: change to get
#[utoipa::path(
    post,
    path = "recommendations",
    request_body = RecommendationRequest,
    responses(
        (status = 200, body = Vec<Recommendation>, description = "Recommendations for a client"),
        (status = "4XX", body = ErrorMessage, description = "Opusi daisy, you messed up"),
        (status = "5XX", body = ErrorMessage, description = "Opusi daisy, we messed up, sorry"),
    )
)]
async fn get_recommendations(
    state: AppState,
    Json(rec): Json<RecommendationRequest>,
) -> AppResult<Vec<Recommendation>> {
    Ok(Json(vec![Recommendation {
        id: 1,
        score: 1.1,
        url: "String".to_owned(),
        image: "String".to_owned(),
        title: "String".to_owned(),
        resume: "String".to_owned(),
    }]))
}
