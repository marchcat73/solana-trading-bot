use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
    http::StatusCode,
};
use std::collections::HashMap;

pub async fn auth_middleware(
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // TODO: Implement authentication
    Ok(next.run(request).await)
}
