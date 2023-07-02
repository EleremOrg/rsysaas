mod data;
mod recsys;
mod web;

use web::routes::routes;

#[tokio::main]
async fn main() {
    axum::Server::bind(&([0, 0, 0, 0], 8000).into())
        .serve(routes().into_make_service())
        .await
        .unwrap();
}
