#[macro_use]
extern crate rbatis;

pub mod common;
pub mod handler;
pub mod middleware;
pub mod model;
pub mod route;
pub mod utils;
pub mod vo;

use axum::{middleware as md, BoxError, Router};

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
use serde::{Deserialize};
use std::sync::Arc;
use std::time::Duration;
use axum::body::Body;
use axum::http::{Method, Request, Response, StatusCode};
use axum::response::IntoResponse;
use tower::{ServiceBuilder};
use tower_http::{
    catch_panic::CatchPanicLayer, classify::ServerErrorsFailureClass, trace::TraceLayer,
};
use tracing::{error, info, Span};
use tracing_subscriber;
use utoipa::{OpenApi};
use utoipa_swagger_ui::SwaggerUi;
use uuid::Uuid;
use utils::db::init_db;
use crate::common::error::AppError;
use crate::middleware::error::{ handle_middleware_error};
use crate::middleware::swagger::swagger_auth;
use axum::routing::get;
use garde::rules::ip::IpKind::Any;
use tower_http::cors::CorsLayer;
use tower_http::timeout::TimeoutLayer;
use tracing_appender::rolling;
use crate::handler::system::sys_user_handler::reset_sys_user_password;

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

#[derive(OpenApi)]
#[openapi(
    paths(
        handler::system::sys_dept_handler::add_sys_dept,
    ),
    components(
        schemas(vo::system::sys_dept_vo::DeleteDeptReq)
    ),
    tags(
            (name = "ToDo App", description = "Todo items management API")
    )
)]
struct ApiDoc;

// 主函数，使用tokio异步运行时
#[tokio::main]
async fn main() {
    // #[cfg(debug_assertions)]
    // {
    //     // 初始化日志配置
    //     log4rs::init_file("src/config/log4rs.yaml", Default::default()).unwrap();
    // }

    // #[cfg(not(debug_assertions))]
    let file_appender = rolling::daily("log", "axum-admin");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let subscriber = tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_ansi(false)
        .with_max_level(tracing::Level::TRACE)
        .with_thread_ids(true)
        .with_thread_names(true)
        .init();

    // 加载和解析配置文件
    let config = Config::builder().add_source(File::with_name("config.toml")).build().unwrap().try_deserialize::<Config1>().unwrap();
    println!("Config: {:?}", config);

    // 初始化数据库连接
    let rb = init_db(config.db.url.as_str()).await;
    let rd = init_redis(config.redis.url.as_str()).await;

    // 创建共享应用状态，包含数据库连接池
    let shared_state = Arc::new(AppState { batis: rb, redis: rd });

    // 跨域中间件
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(tower_http::cors::Any)
        .allow_headers(tower_http::cors::Any);

    // Swagger-UI
    let swagger_ui = SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi());

    // 请求超时中间件
    let timeout = TimeoutLayer::new(Duration::from_secs(3));

    // 请求追踪中间件
    let trace = TraceLayer::new_for_http()
        .make_span_with(|_request: &Request<Body>| {
                let request_id = Uuid::new_v4().to_string();
                tracing::info_span!("http-request: ", %request_id)
            }
        )
        .on_request(|request: &Request<Body>, _span: &Span| {
                info!("request: {} {}", request.method(), request.uri().path())
            }
        )
        .on_response(|response: &Response<Body>, latency: Duration, _span: &Span| {
                info!("response: {} {:?}", response.status(), latency);
            },
        )
        .on_failure(|err: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
                error!("Don't panic, C'est la vie. {}", err)
            },
        );

    // 全局异常捕获中间件
    let panic = CatchPanicLayer::custom(|panic_info: Box<dyn std::any::Any + Send>| {
        // 这里可以上报日志、监控或做其他操作
        eprintln!("Custom panic hook: {panic_info:?}");
        AppError::default().into_response()
    });

    // 首页路由
    let index_router = Router::new().route("/", get(async||-> &'static str {
        "Hello axum-admin!"
    }));

    // 构建应用路由，并合并多个子路由
    let app = Router::new().merge(swagger_ui).merge(index_router)//.route_layer(md::from_fn(swagger_auth))
        .nest(
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
            .layer(cors)
            .layer(timeout) // 设置请求超时时间为3秒
            .layer(trace)
            .layer(panic)
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