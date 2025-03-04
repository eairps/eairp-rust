use axum::{Json, Router};
use domain::dto::user::account_login_dto::AccountLoginDTO;
use domain::vo::user::user_info_vo::UserInfoVO;
use service::user::user_service::{AuthError, UserService};
use std::sync::Arc;
use axum::routing::{post};
use axum_macros::debug_handler;

// 用户路由模块
pub fn user_routes() -> Router {
    Router::new()
        .route("/user/login", post(account_login))
}

#[debug_handler]
pub async fn account_login(Json(payload): Json<AccountLoginDTO>) -> Result<Json<UserInfoVO>, AuthError> {
    let service = Arc::new(UserService::new());
    service.user_login(payload).await
}