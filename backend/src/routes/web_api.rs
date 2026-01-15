use crate::{
    adapters::gemini_response::GeminiQueryProvider,
    handlers::{self},
    types::AppState,
};
use axum::{
    response::Html,
    routing::{get, post},
};
use http::Method;
use http::header::{ACCEPT, ACCESS_CONTROL_ALLOW_ORIGIN, AUTHORIZATION, CONTENT_TYPE};
use tower_http::cors::{Any, CorsLayer};

pub async fn web_api() -> axum::Router {
    let ai_chat_provider = GeminiQueryProvider::new("your_api_key".to_string());
    let app_state: AppState = AppState { ai_chat_provider };
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([
            AUTHORIZATION,
            ACCEPT,
            CONTENT_TYPE,
            ACCESS_CONTROL_ALLOW_ORIGIN,
        ])
        .allow_origin(Any);
    axum::Router::new()
        .route("/", get(handlers::first_route::first_route))
        .route(
            "/hello",
            get(|| async { Html("<h1>Hello from Web API!</h1>") }),
        )
        .route("/get-answer", post(handlers::chat_bot::get_answer_from_ai))
        .with_state(app_state)
        .layer(cors)
}
