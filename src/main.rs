#[macro_use]
extern crate rbatis;

pub mod common;
pub mod handler;
pub mod middleware;
pub mod model;
pub mod route;
pub mod utils;
pub mod vo;

use axum::{middleware as md, Router};

use crate::model::db::init_db;
use crate::route::system::sys_dept_route::build_sys_dept_route;
use crate::route::system::sys_dict_data_route::build_sys_dict_data_route;
use crate::route::system::sys_dict_type_route::build_sys_dict_type_route;
use crate::route::system::sys_login_log_route::build_sys_login_log_route;
use crate::route::system::sys_notice_route::build_sys_notice_route;
use crate::route::system::sys_operate_log_route::build_sys_operate_log_route;
use crate::route::system::sys_post_route::build_sys_post_route;
use middleware::auth::auth;
use rbatis::RBatis;
use route::system::sys_menu_route::build_sys_menu_route;
use route::system::sys_role_route::build_sys_role_route;
use route::system::sys_user_route::build_sys_user_route;
use std::sync::Arc;

pub struct AppState {
    pub batis: RBatis,
}

#[tokio::main]
async fn main() {
    log4rs::init_file("src/config/log4rs.yaml", Default::default()).unwrap();
    let rb = init_db().await;
    let shared_state = Arc::new(AppState {
        /* ... */ batis: rb.clone(),
    });

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
            .route_layer(md::from_fn(auth))
            .with_state(shared_state),
    );

    // axum 0.6.x
    // let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    // log::info!("listening on {}", addr);
    // axum::Server::bind(&addr)
    //     .serve(app.into_make_service())
    //     .await
    //     .unwrap();

    // axum 0.7.x
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
