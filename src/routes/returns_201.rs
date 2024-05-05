use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub async fn returns_201() -> Response {
    (StatusCode::CREATED, "(this is 201 in tuple)").into_response()
}