use axum::{Router, Json};
use axum::routing::post;
use service::tenant::tenant_service::TenantService;

pub fn tenant_routes() -> Router {
    Router::new()
        .route("/tenant", post(create_dept))
}

async fn create_dept() -> Result<Json<String>, ()> {
    let tenant_service = TenantService::new();
    Ok(Json("Hello".to_string()))
}