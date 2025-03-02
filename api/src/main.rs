mod user;

use axum::Router;
use axum::routing::post;
use tokio;
use utils::config::load_config;
use user::auth;

/// # Eairp api v2 Application Main function
/// @author James Zow
///
/// @date 2024-12-02
///
/// @description
/// This is the main function of the project, which is used to start the web server.
///
#[tokio::main]
async fn main() {
    // load config
    let config = load_config();

    // init app router
    let app = Router::new()
        .route("/api/user/login", post(auth::account_login));

    // print server info
    println!("Server is running on http://{}:{}", config.server.host, config.server.port);

    // run server
    let listener = tokio::net::TcpListener::bind(&format!(
        "{}:{}", config.server.host, config.server.port))
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
