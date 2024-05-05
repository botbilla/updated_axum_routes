use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MirrorJson{
    message: String
}

#[derive(Serialize)]
pub struct MirrorJsonResponse{
    message: String,
    message_server: String,
}

pub async fn mirror_body_json(Json(body): Json<MirrorJson>) -> Json<MirrorJsonResponse>{
    Json(MirrorJsonResponse{
        message: body.message,
        message_server: "hello from Axum".to_string()
    })
}