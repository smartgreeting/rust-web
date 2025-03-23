/*
 * @Author: lihuan
 * @Date: 2025-03-21 22:38:36
 * @LastEditors: lihuan
 * @LastEditTime: 2025-03-23 22:09:57
 * @Email: 17719495105@163.com
 */
use std::net::SocketAddr;

use axum::Extension;
use sqlx::postgres::PgPoolOptions;
use tower_http::trace::TraceLayer;


mod config;
mod routes;
mod error;
mod utils;
mod middleware;



pub async fn run() {
   // 1. 初始化配置
   let app_config = config::AppConfig::load()
   .unwrap_or_else(|err| {
      eprintln!("❌ 配置加载失败: {}", err);
      eprintln!("💡 检查路径: {:?}", std::env::current_dir().unwrap().join("config"));
      std::process::exit(1);
  });

// 2. 初始化日志
tracing_subscriber::fmt()
   .with_max_level(tracing::Level::INFO)
   .init();

// 3. 初始化数据库连接池
// let db_pool = PgPoolOptions::new()
//    .max_connections(app_config.database.max_connections)
//    .connect(&app_config.database.url)
//    .await
//    .expect("Failed to create database pool");

// 4. 初始化工具类
let jwt_util =  utils::jwt::JwtUtil::new(&app_config.auth.jwt_secret);

// 5. 构建路由
// let app = routes::init_router().layer(
//    tower::ServiceBuilder::new()
//             .layer(TraceLayer::new_for_http())
//             .layer(middleware::rate_limite::rate_limiter(&app_config))
//             .layer(middleware::circuit_breaker::circuit_breaker())
//             .layer(Extension(jwt_util))
//             .layer(Extension(db_pool))
//             .into_inner()
// );
let app = routes::init_router().layer(Extension(jwt_util));


// 6. 启动服务
let addr = "0.0.0.0:3000".parse::<SocketAddr>().unwrap();
let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

tracing::info!("Server running on {}", addr);
axum::serve(listener,app)
   .await
   .unwrap();
}
