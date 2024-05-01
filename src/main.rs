use axum::{
    http::{HeaderValue, Method},
    routing::{get, post},
    Router,
};
use example_api_rust::handlers;
use std::env;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let tcp_address = env::var("BIND_ADDRESS").unwrap_or_else(|_| "0.0.0.0:3000".to_string());
    let app = router().layer(cors_layer());
    let listener = tokio::net::TcpListener::bind(tcp_address).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

fn router() -> Router {
    Router::new()
        .route("/", get(handlers::root))
        .route("/creature", get(handlers::get_creature))
        .route("/users", post(handlers::create_user))
}

fn cors_layer() -> CorsLayer {
    let cors_allow_origin_address =
        env::var("CORS_ORIGIN").unwrap_or_else(|_| "http://localhost:5173".to_string());
    CorsLayer::new()
        .allow_origin(cors_allow_origin_address.parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET])
}
