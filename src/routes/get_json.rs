use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Data{
    message: String,
    count: i32,
    username: String,
}

pub async fn get_json() -> Json<Data>{
    let data = Data{
        message: "I'm a json return".to_owned(),
        count: 200,
        username: "botbilla".to_owned(),
    };
    Json(data)
}