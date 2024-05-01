use crate::models::{CreateUser, Creature, User};
use axum::{http::StatusCode, Json};

pub async fn root() -> &'static str {
    "Hello from Rust!"
}

pub async fn get_creature() -> (StatusCode, Json<Creature>) {
    let creature = Creature::new(String::from("Goat"), String::from("Billy"), 2, 2);
    (StatusCode::OK, Json(creature))
}

pub async fn create_user(Json(payload): Json<CreateUser>) -> (StatusCode, Json<User>) {
    let user = User {
        id: 1337,
        username: payload.username,
    };
    (StatusCode::CREATED, Json(user))
}

#[cfg(test)]
mod handlers_tests {
    use super::*;

    #[tokio::test]
    async fn test_root_returns_hello() {
        let response = root().await;
        assert_eq!(response, "Hello from Rust!");
    }

    #[tokio::test]
    async fn test_get_creature_returns_ok() {
        let (status, _body) = get_creature().await;
        assert_eq!(status, StatusCode::OK);
    }

    #[tokio::test]
    async fn test_create_user_returns_created() {
        const TEST_USER: &str = "test_user";
        let payload = CreateUser {
            username: String::from(TEST_USER),
        };

        let (status, response) = {
            let payload = axum::extract::Json(payload); // Convert payload into axum Json
            create_user(payload).await
        };

        assert_eq!(response.username, TEST_USER);
        assert_eq!(status, StatusCode::CREATED);
    }
}
