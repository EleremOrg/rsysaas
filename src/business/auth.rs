use axum::http::HeaderMap;
use headers::HeaderValue;

pub async fn get_bearer_token<'a>(headers: &'a HeaderMap) -> Option<&'a str> {
    match headers.get("authorization") {
        Some(value) => extract_bearer_token(value).await,
        None => None,
    }
}

async fn extract_bearer_token<'a>(header_value: &'a HeaderValue) -> Option<&'a str> {
    let token = match header_value.to_str() {
        Ok(token) => token,
        Err(_) => return None,
    };
    let mut parts = token.split_whitespace();
    if parts.next() == Some("Bearer") {
        parts.next()
    } else {
        None
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_bearer_token() {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            HeaderValue::from_static("Bearer your_token"),
        );

        let result = get_bearer_token(&headers).await;
        assert_eq!(result, Some("your_token"));
    }

    #[tokio::test]
    async fn test_get_bearer_token_no_authorization_header() {
        let headers = HeaderMap::new();

        let result = get_bearer_token(&headers).await;
        assert_eq!(result, None);
    }

    #[tokio::test]
    async fn test_get_bearer_token_invalid_header_value() {
        let mut headers = HeaderMap::new();
        headers.insert("Authorization", HeaderValue::from_static("InvalidHeader"));

        let result = get_bearer_token(&headers).await;
        assert_eq!(result, None);
    }

    #[tokio::test]
    async fn test_extract_bearer_token() {
        let header_value = HeaderValue::from_static("Bearer your_token");

        let result = extract_bearer_token(&header_value).await;
        assert_eq!(result, Some("your_token"));
    }

    #[tokio::test]
    async fn test_extract_bearer_token_invalid_header_value() {
        let header_value = HeaderValue::from_static("InvalidHeader");

        let result = extract_bearer_token(&header_value).await;
        assert_eq!(result, None);
    }
}
