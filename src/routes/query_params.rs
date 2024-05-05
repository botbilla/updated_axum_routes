use axum::extract::Query;
use axum::Json;
use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize)]
pub struct QueryParams{
    message: String,
    id: i32,
}

pub async fn query_params(Query(query): Query<QueryParams>) -> Json<QueryParams>{
    Json(query)
}