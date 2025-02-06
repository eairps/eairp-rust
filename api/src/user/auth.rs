use axum::Json;
use domain::dto::user::account_login_dto::AccountLoginDTO;
use domain::vo::user::user_info_vo::UserInfoVO;
use service::user::user_service::{AuthError, UserService};
use std::sync::Arc;
use axum_macros::debug_handler;

#[debug_handler]
pub async fn account_login(Json(payload): Json<AccountLoginDTO>) -> Result<Json<UserInfoVO>, AuthError> {
    let service = Arc::new(UserService::new());
    service.user_login(payload).await
}