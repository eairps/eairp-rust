mod config;

use axum::{Router, routing};
use tokio;
use config::load_config;

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
    // 加载配置
    let config = load_config();

    // 初始化应用路由
    let app = Router::new();

    // 输出服务器启动信息
    println!("Server is running on http://{}:{}", config.server.host, config.server.port);

    // run server
    let listener = tokio::net::TcpListener::bind(&format!(
        "{}:{}", config.server.host, config.server.port))
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
