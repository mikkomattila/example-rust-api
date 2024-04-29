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
    use axum::http::StatusCode;

    #[tokio::test]
    async fn test_root() {
        let response = root().await;
        assert_eq!(response, "Hello from Rust!");
    }

    #[tokio::test]
    async fn test_get_creature() {
        let (status, _body) = get_creature().await;
        assert_eq!(status, StatusCode::OK);
    }
}
