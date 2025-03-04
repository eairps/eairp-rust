mod user;
mod tenant;

use axum::Router;

pub fn all_routes() -> Router {
    Router::new()
        .merge(user::user_routes())
        .merge(tenant::tenant_routes())
}