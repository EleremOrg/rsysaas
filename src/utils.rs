use serde::{de::DeserializeOwned, Serialize};
use stefn::AppError;

pub async fn post_request<T: Serialize, R: DeserializeOwned>(
    client: &reqwest::Client,
    paylod: &T,
    url: &str,
) -> Result<R, AppError> {
    request(client.post(url).json(paylod)).await
}

pub async fn request<R: DeserializeOwned>(request: reqwest::RequestBuilder) -> Result<R, AppError> {
    request
        .send()
        .await
        .map_err(|err| AppError::custom_internal(&err.to_string()))? //TODO: only log the errors do not do that
        .json()
        .await
        .map_err(|err| AppError::custom_internal(&err.to_string()))
}
