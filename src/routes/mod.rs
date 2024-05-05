mod hello_world;
mod mirror_body_string;
mod mirror_body_json;
mod path_variables;
mod query_params;
mod mirror_user_agents;
mod middleware_message;
mod read_middleware_custom_header;
mod set_middleware_custom_header;
mod always_errors;
mod returns_201;
mod get_json;

use axum::{Extension, Router, routing::{get, post}};
use axum::http::Method;
use axum::middleware;

use hello_world::my_function;
use mirror_body_string::mirror_body_string;
use mirror_body_json::mirror_body_json;
use path_variables::path_variables;
use query_params::query_params;
use mirror_user_agents::mirror_user_agents;
use middleware_message::middleware_message;
use tower_http::cors::CorsLayer;
use tower_http::cors::Any;
use read_middleware_custom_header::read_middleware_custom_header;
use set_middleware_custom_header::set_middleware_custom_header;
use always_errors::always_errors;
use returns_201::returns_201;
use get_json::get_json;


#[derive(Clone)]
pub struct SharedData{
    pub message: String,
}

pub fn create_routes() -> Router{
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);
    let shared_data = SharedData{
        message: "Hello from shared data".to_owned(),
    };

    Router::new().route("/hello", get(my_function))
        .route(
            "/read_middleware_custom_header",
            get(read_middleware_custom_header)
        )
        .route_layer(middleware::from_fn(set_middleware_custom_header))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json))
        .route("/path_variable/:id", get(path_variables))
        .route("/query_params", get(query_params))
        .route("/mirror_user_agents", get(mirror_user_agents))
        .route("/middleware_message", get(middleware_message))
        .layer(cors)
        .layer(Extension(shared_data))
        .route("/always_errors", get(always_errors))
        .route("/returns_201", post(returns_201))
        .route("/get_json", get(get_json))
}