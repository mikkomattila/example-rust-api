use axum::{
    http::{HeaderValue, Method},
    routing::{get, post},
    Router,
};
use example_api_rust::handlers;
use sqlx::postgres::{PgPoolOptions, Postgres};
use sqlx::{Error, Pool};
use std::env;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Clean this up
    let postgres_user = env_var("POSTGRES_USER").unwrap_or_else(|_| "postgres".to_string());
    let postgres_password = env_var("POSTGRES_PASSWORD").unwrap_or_else(|_| "postgres".to_string());
    let postgres_hostname =
        env_var("POSTGRES_HOSTNAME").unwrap_or_else(|_| "localhost".to_string());
    let postgres_port = env_var("POSTGRES_PORT").unwrap_or_else(|_| "5432".to_string());
    let postgres_db = env_var("POSTGRES_DB").unwrap_or_else(|_| "postgres".to_string());

    let connection_string = &format!(
        "postgres://{}:{}@{}:{}/{}",
        postgres_user, postgres_password, postgres_hostname, postgres_port, postgres_db
    );

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(connection_string)
        .await
        .expect("Failed to connect to database");

    let cors_allow_origin =
        env_var("CORS_ALLOW_ORIGIN").unwrap_or_else(|_| "http://localhost:5173".to_string());
    let tcp_address = env_var("TCP_BIND_ADDRESS").unwrap_or_else(|_| "0.0.0.0:3000".to_string());

    let app = router(pool).layer(cors_layer(cors_allow_origin));
    let listener = tokio::net::TcpListener::bind(tcp_address)
        .await
        .expect("Failed to bind TcpListener");

    axum::serve(listener, app).await.unwrap();

    Ok(())
}

fn env_var(name: &str) -> Result<String, Error> {
    env::var(name).map_err(|e| Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))
}

fn router(pool: Pool<Postgres>) -> Router {
    Router::new()
        .route("/", get(handlers::root))
        .with_state(pool)
        .route("/creature", get(handlers::get_creatures))
        .route("/users", post(handlers::create_user))
}

fn cors_layer(cors_allow_origin: String) -> CorsLayer {
    CorsLayer::new()
        .allow_origin(
            cors_allow_origin
                .parse::<HeaderValue>()
                .expect("Failed to parse cors allow origin value."),
        )
        .allow_methods([Method::GET])
}
