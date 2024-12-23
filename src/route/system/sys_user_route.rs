use crate::handler::system::sys_user_handler;
use crate::AppState;
use axum::routing::{get, post};
use axum::Router;
use std::sync::Arc;

/*
 *构建用户信息路由
 *author：刘飞华
 *date：2024/12/12 17:04:49
 */
pub fn build_sys_user_route() -> Router<Arc<AppState>> {
    Router::new()
        .route("/system/user/addUser", post(sys_user_handler::add_sys_user))
        .route(
            "/system/user/deleteUser",
            post(sys_user_handler::delete_sys_user),
        )
        .route(
            "/system/user/updateUser",
            post(sys_user_handler::update_sys_user),
        )
        .route(
            "/system/user/updateUserStatus",
            post(sys_user_handler::update_sys_user_status),
        )
        .route(
            "/system/user/queryUserDetail",
            post(sys_user_handler::query_sys_user_detail),
        )
        .route(
            "/system/user/queryUserList",
            post(sys_user_handler::query_sys_user_list),
        )
        .route("/system/user/login", post(sys_user_handler::login))
        .route(
            "/system/user/queryUserMenu",
            get(sys_user_handler::query_user_menu),
        )
        .route(
            "/system/user/queryUserRole",
            get(sys_user_handler::query_user_role),
        )
        .route(
            "/system/user/updateUserRole",
            post(sys_user_handler::update_user_role),
        )
        .route(
            "/system/user/updateUserPassword",
            post(sys_user_handler::update_sys_user_password),
        )
    //记得在main.rs中添加路由build_sys_user_route()
}
