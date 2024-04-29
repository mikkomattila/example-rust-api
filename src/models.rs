use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Creature {
    pub creature_type: String,
    pub name: String,
    pub arms: i32,
    pub legs: i32,
}

impl Creature {
    pub fn new(creature_type: String, name: String, arms: i32, legs: i32) -> Creature {
        Creature {
            creature_type,
            name,
            arms,
            legs,
        }
    }
}

#[derive(Deserialize)]
pub struct CreateUser {
    pub username: String,
}

#[derive(Serialize)]
pub struct User {
    pub id: u64,
    pub username: String,
}
