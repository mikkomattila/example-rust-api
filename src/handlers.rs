use crate::models::{CreateUser, Creature, User};
use axum::{extract::State, http::StatusCode, Json};
use sqlx::postgres::PgPool;

/// Random hard coded endpoints for practice

pub async fn root(State(pool): State<PgPool>) -> Result<String, (StatusCode, String)> {
    sqlx::query_scalar("select 'Hello World from Postgres!'")
        .fetch_one(&pool)
        .await
        .map_err(internal_error)
}

pub async fn get_creatures() -> (StatusCode, Json<Vec<Creature>>) {
    let creatures = vec![
        Creature::new(String::from("Goat"), String::from("Billy"), 2, 2),
        Creature::new(String::from("Goat"), String::from("Bronson"), 4, 0),
    ];

    (StatusCode::OK, Json(creatures))
}

pub async fn create_user(Json(payload): Json<CreateUser>) -> (StatusCode, Json<User>) {
    let user = User {
        id: 1337,
        username: payload.username,
    };
    (StatusCode::CREATED, Json(user))
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

#[cfg(test)]
mod handlers_tests {
    use super::*;

    // #[tokio::test]
    // async fn test_root_returns_hello() {
    //     let response = root().await;
    //     assert_eq!(response, "Hello from Rust!");
    // }

    #[tokio::test]
    async fn test_get_creatures_returns_ok() {
        let (status, _body) = get_creatures().await;
        assert_eq!(status, StatusCode::OK);
    }

    #[tokio::test]
    async fn test_create_user_returns_created() {
        const TEST_USER: &str = "test_user";
        let payload = CreateUser {
            username: String::from(TEST_USER),
        };

        let (status, response) = { create_user(axum::extract::Json(payload)).await };

        assert_eq!(response.username, TEST_USER);
        assert_eq!(status, StatusCode::CREATED);
    }
}
