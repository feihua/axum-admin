use std::sync::Arc;
use axum::Router;
use axum::routing::post;
use crate::AppState;
use crate::handler::system::sys_menu_handler;

/*
 *构建菜单信息路由
 *author：刘飞华
 *date：2024/12/12 17:04:49
 */
pub fn build_sys_menu_route() -> Router<Arc<AppState>> {
    Router::new()
        .route("/add_menu", post(sys_menu_handler::add_sys_menu))
        .route("/delete_menu", post(sys_menu_handler::delete_sys_menu))
        .route("/update_menu", post(sys_menu_handler::update_sys_menu))
        .route("/update_menu_status", post(sys_menu_handler::update_sys_menu_status))
        .route("/query_menu_detail", post(sys_menu_handler::query_sys_menu_detail))
        .route("/query_menu_list", post(sys_menu_handler::query_sys_menu_list))
        //记得在main.rs中添加路由build_sys_menu_route()
}
