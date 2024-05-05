use axum::extract::Request;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::Response;
// use crate::routes::read_middleware_custom_header::HeaderMessage;
use super::read_middleware_custom_header::HeaderMessage;

pub async fn set_middleware_custom_header(
    mut request: Request, next: Next
) -> Result<Response, StatusCode> {

    let headers = request.headers();
    let message = headers
        .get("message")
        .ok_or_else(|| StatusCode::BAD_REQUEST)?;
    let message = message
        .to_str()
        .map_err(|_error| StatusCode::BAD_REQUEST)?
        .to_owned();
    let extentions = request.extensions_mut();
    extentions.insert(HeaderMessage(message));
    Ok(next.run(request).await)
}