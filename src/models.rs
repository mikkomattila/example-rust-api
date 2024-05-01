use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Creature {
    pub creature_type: String,
    pub name: String,
    pub arms: u32,
    pub legs: u32,
}

impl Creature {
    pub fn new(creature_type: String, name: String, arms: u32, legs: u32) -> Creature {
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
