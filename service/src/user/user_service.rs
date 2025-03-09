use dao::user::user_repository::find_user_by_credentials;
use domain::dto::user::account_login_dto::AccountLoginDTO;
use domain::vo::user::user_info_vo::UserInfoVO;
use base64::engine::general_purpose::STANDARD;
use base64::Engine as _;
use serde_json::json;
use md5;
use aes::Aes128;
use cipher::{KeyIvInit, BlockDecryptMut};
use cbc::cipher::block_padding::Pkcs7;
use cbc::Decryptor;
use axum::http::StatusCode;
use axum::{Json, Router};
use axum::response::{IntoResponse, Response};
use utils::constants::security_constants::security_key;
use utils::redis::create_token;

type Aes128CbcDec = Decryptor<Aes128>;

const IV: &[u8; 16] = b"1234567890123456"; // 请根据实际情况调整 IV

#[derive(Debug)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };
        let body = Json(json!({
            "msg": error_message,
        }));
        (status, body).into_response()
    }
}

pub struct UserService;

impl UserService {
    pub fn new() -> Self {
        UserService
    }

    pub async fn user_login(&self, account_login_dto: AccountLoginDTO) -> Result<Json<UserInfoVO>, AuthError>  {
        // Check if username or password is empty
        if account_login_dto.username.is_empty() || account_login_dto.password.is_empty() {
            return Err(AuthError::MissingCredentials);
        }

        // Decrypt the password
        let encrypted_password = match STANDARD.decode(&account_login_dto.password) {
            Ok(pwd) => pwd,
            Err(_) => {
                return Err(AuthError::WrongCredentials);
            }
        };

        let cipher = match Aes128CbcDec::new_from_slices(security_key::LOGIN.as_ref(), IV) {
            Ok(cipher) => cipher,
            Err(_) => {
                return Err(AuthError::TokenCreation);
            }
        };

        let mut buffer = encrypted_password.to_vec();
        let decrypted_password = match cipher.decrypt_padded_mut::<Pkcs7>(&mut buffer) {
            Ok(pwd) => pwd,
            Err(_) => {
                return Err(AuthError::MissingCredentials);
            }
        };

        let decrypted_password_str = match String::from_utf8(decrypted_password.to_vec()) {
            Ok(pwd) => pwd,
            Err(_) => {
                return Err(AuthError::MissingCredentials);
            }
        };

        // Get user by username and password
        let hashed_password = format!("{:x}", md5::compute(decrypted_password_str));
        match find_user_by_credentials(&account_login_dto.username, &*account_login_dto.password).await {
            Ok(Some(user)) => {
                let token = match create_token(&account_login_dto.username) {
                    Ok(token) => token,
                    Err(_) => {
                        return Err(AuthError::MissingCredentials);
                    }
                };
                let user_info = UserInfoVO {
                    id: user.id,
                    name: user.name.clone(),
                    position: None,
                    email: user.email.clone(),
                    phone_number: None,
                    description: None,
                    user_name: None,
                    avatar: None,
                    system_language: None,
                    token: Some(token),
                    expire: Some(1694757956),
                };
                Ok(Json(user_info))
            }
            _ => Err(AuthError::MissingCredentials)
        }
    }
}