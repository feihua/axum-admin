#[macro_use]
extern crate rbatis;

pub mod app_config;
pub mod common;
pub mod handler;
pub mod middleware;
pub mod model;
pub mod route;
pub mod service;
pub mod utils;
pub mod vo;

use axum::{middleware as md, Router};

use crate::app_config::config::JwtConfig;
use crate::app_config::load_config::init_config;
use crate::app_config::load_log::init_log;
use crate::route::system::sys_dept_route::build_sys_dept_route;
use crate::route::system::sys_dict_data_route::build_sys_dict_data_route;
use crate::route::system::sys_dict_type_route::build_sys_dict_type_route;
use crate::route::system::sys_login_log_route::build_sys_login_log_route;
use crate::route::system::sys_notice_route::build_sys_notice_route;
use crate::route::system::sys_operate_log_route::build_sys_operate_log_route;
use crate::route::system::sys_post_route::build_sys_post_route;
use crate::utils::redis_util::init_redis;
use middleware::auth::auth;
use rbatis::RBatis;
use redis::Client;
use route::system::sys_menu_route::build_sys_menu_route;
use route::system::sys_role_route::build_sys_role_route;
use route::system::sys_user_route::build_sys_user_route;
use std::sync::Arc;
use utils::db::init_db;

// 定义应用状态结构体，包含数据库连接池
pub struct AppState {
    pub batis: RBatis,
    pub redis: Client,
    pub jwt: JwtConfig,
}

#[tokio::main]
async fn main() {
    // 1.加载和解析配置文件
    let config = init_config("config.toml");
    // 2.初始化日志
    let _guard = init_log(config.log);

    // 初始化数据库连接
    let rb = init_db(config.db.url.as_str()).await;
    let rd = init_redis(config.redis.url.as_str()).await;

    // 创建共享应用状态，包含数据库连接池
    let shared_state = Arc::new(AppState {
        batis: rb,
        redis: rd,
        jwt: config.jwt,
    });

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

    // 创建TCP监听器
    let listener = tokio::net::TcpListener::bind(config.server.addr).await.unwrap();
    // 使用监听器启动服务器
    axum::serve(listener, app).await.unwrap();
}
