#[macro_use]
extern crate rbatis;

pub mod common;
pub mod handler;
pub mod middleware;
pub mod model;
pub mod route;
pub mod service;
pub mod utils;
pub mod vo;

use axum::{middleware as md, Router};

use crate::route::system::sys_dept_route::build_sys_dept_route;
use crate::route::system::sys_dict_data_route::build_sys_dict_data_route;
use crate::route::system::sys_dict_type_route::build_sys_dict_type_route;
use crate::route::system::sys_login_log_route::build_sys_login_log_route;
use crate::route::system::sys_notice_route::build_sys_notice_route;
use crate::route::system::sys_operate_log_route::build_sys_operate_log_route;
use crate::route::system::sys_post_route::build_sys_post_route;
use crate::utils::redis_util::init_redis;
use config::{Config, File};
use middleware::auth::auth;
use rbatis::RBatis;
use redis::Client;
use route::system::sys_menu_route::build_sys_menu_route;
use route::system::sys_role_route::build_sys_role_route;
use route::system::sys_user_route::build_sys_user_route;
use serde::Deserialize;
use std::sync::Arc;
use tracing_appender::rolling;
use tracing_subscriber::fmt::time::ChronoLocal;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, Layer};
use utils::db::init_db;

// 定义应用状态结构体，包含数据库连接池
pub struct AppState {
    pub batis: RBatis,
    pub redis: Client,
}

// 配置结构体，包含服务器和数据库配置
#[derive(Debug, Deserialize)]
struct Config1 {
    server: ServerConfig,
    db: DbConfig,
    redis: RedisConfig,
}

// 服务器配置结构体，包含服务器地址
#[derive(Debug, Deserialize)]
struct ServerConfig {
    addr: String,
}

// 数据库配置结构体，包含数据库URL
#[derive(Debug, Deserialize)]
struct DbConfig {
    url: String,
}

#[derive(Debug, Deserialize)]
struct RedisConfig {
    url: String,
}
// 主函数，使用tokio异步运行时
#[tokio::main]
async fn main() {
    // 初始化日志配置
    let file_appender = rolling::daily("logs", "app.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    let timer = ChronoLocal::new("%Y-%m-%d %H:%M:%S%.3f".to_string());
    let format = fmt::format().with_target(false).with_timer(timer).with_file(true).with_line_number(true);

    let console_layer = fmt::layer()
        .event_format(format.clone())
        .with_writer(std::io::stdout)
        .with_filter(tracing_subscriber::filter::LevelFilter::DEBUG);
    let file_layer = fmt::layer()
        .event_format(format.clone())
        .with_writer(non_blocking)
        .with_filter(tracing_subscriber::filter::LevelFilter::DEBUG);

    tracing_subscriber::registry().with(console_layer).with(file_layer).init();

    // 加载和解析配置文件
    let config = Config::builder().add_source(File::with_name("config.toml")).build().unwrap().try_deserialize::<Config1>().unwrap();
    println!("Config: {:?}", config);

    // 初始化数据库连接
    let rb = init_db(config.db.url.as_str()).await;
    let rd = init_redis(config.redis.url.as_str()).await;

    // 创建共享应用状态，包含数据库连接池
    let shared_state = Arc::new(AppState { batis: rb, redis: rd });

    // 构建应用路由，并合并多个子路由
    let app = Router::new().nest(
        "/api",
        Router::new()
            .merge(build_sys_user_route())
            .merge(build_sys_role_route())
            .merge(build_sys_menu_route())
            .merge(build_sys_dept_route())
            .merge(build_sys_dict_type_route())
            .merge(build_sys_dict_data_route())
            .merge(build_sys_post_route())
            .merge(build_sys_login_log_route())
            .merge(build_sys_operate_log_route())
            .merge(build_sys_notice_route())
            .route_layer(md::from_fn_with_state(Arc::clone(&shared_state), auth)) // 添加认证中间件
            .with_state(shared_state), // 设置共享状态
    );

    // 以下代码适用于axum 0.6.x版本
    // 定义服务器监听地址
    // let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    // log::info!("listening on {}", addr);
    // 使用绑定地址启动服务器
    // axum::Server::bind(&addr)
    //     .serve(app.into_make_service())
    //     .await
    //     .unwrap();

    // 以下代码适用于axum 0.7.x版本
    // 创建TCP监听器
    let listener = tokio::net::TcpListener::bind(config.server.addr).await.unwrap();
    // 使用监听器启动服务器
    axum::serve(listener, app).await.unwrap();
}
