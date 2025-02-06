use serde::{Deserialize};

#[derive(Deserialize)]
pub struct AccountLoginDTO {
    pub username: String,
    pub password: String,
}