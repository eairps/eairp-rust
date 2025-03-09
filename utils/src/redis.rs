use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};
use jsonwebtoken::{encode, Header, EncodingKey};
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use crate::constants::security_constants::{
    security_key
};

#[derive(serde::Serialize)]
struct Claims {
    sub: String,
    exp: usize,
}

async fn create_redis_pool() -> bb8::Pool<RedisConnectionManager> {
    let manager = RedisConnectionManager::new("redis://127.0.0.1/")
        .expect("Invalid Redis URL");
    Pool::builder().build(manager).await.unwrap()
}

pub fn create_token(username: &str) -> Result<String, Box<dyn Error>> {
    let exp = SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_secs() + 86400;

    let claims = Claims {
        sub: username.to_owned(),
        exp: exp as usize,
    };

    encode(&Header::default(),
           &claims,
           &EncodingKey::from_secret(security_key::LOGIN.as_ref()))
        .map_err(|e| e.into())
}