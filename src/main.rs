use axum::{
    http::{HeaderValue, Method},
    routing::{get, post},
    Router,
};
use example_api_rust::handlers;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let app = router().layer(cors_layer());
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

fn router() -> Router {
    Router::new()
        .route("/", get(handlers::root))
        .route("/creature", get(handlers::get_creature))
        .route("/users", post(handlers::create_user))
}

fn cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET])
}
