use std::sync::Arc;
use axum::Router;
use axum::routing::post;
use crate::AppState;
use crate::handler::system::sys_role_handler;

/*
 *构建角色信息路由
 *author：刘飞华
 *date：2024/12/12 17:04:49
 */
pub fn build_sys_role_route() -> Router<Arc<AppState>> {

    Router::new()
        .route("/add_role", post(sys_role_handler::add_sys_role))
        .route("/delete_role", post(sys_role_handler::delete_sys_role))
        .route("/update_role", post(sys_role_handler::update_sys_role))
        .route("/update_role_status", post(sys_role_handler::update_sys_role_status))
        .route("/query_role_detail", post(sys_role_handler::query_sys_role_detail))
        .route("/query_role_list", post(sys_role_handler::query_sys_role_list))
        .route("/query_role_menu", post(sys_role_handler::query_role_menu))
        .route("/update_role_menu", post(sys_role_handler::update_role_menu))
        //记得在main.rs中添加路由build_sys_role_route()
}
