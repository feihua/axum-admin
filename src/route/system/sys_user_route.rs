use std::sync::Arc;
use axum::Router;
use axum::routing::{get, post};
use crate::AppState;
use crate::handler::system::sys_user_handler;

/*
 *构建用户信息路由
 *author：刘飞华
 *date：2024/12/12 17:04:49
 */
pub fn build_sys_user_route() -> Router<Arc<AppState>> {
    Router::new()
        .route("/add_user", post(sys_user_handler::add_sys_user))
        .route("/delete_user", post(sys_user_handler::delete_sys_user))
        .route("/update_user", post(sys_user_handler::update_sys_user))
        .route("/update_user_status", post(sys_user_handler::update_sys_user_status))
        .route("/query_user_detail", post(sys_user_handler::query_sys_user_detail))
        .route("/query_user_list", post(sys_user_handler::query_sys_user_list))
        .route("/login", post(sys_user_handler::login))
        .route("/query_user_menu", get(sys_user_handler::query_user_menu))
        .route("/query_user_role", get(sys_user_handler::query_user_role))
        .route("/update_user_role", get(sys_user_handler::update_user_role))
        .route("/update_user_password", get(sys_user_handler::update_sys_user_password))
        //记得在main.rs中添加路由build_sys_user_route()
}
