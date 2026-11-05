use crate::AppState;
use crate::handler::system::sys_user_handler;
use axum::Router;
use route_macro::routes;
use std::sync::Arc;
/*
 *构建用户信息路由
 *author：刘飞华
 *date：2024/12/12 17:04:49
 */
pub fn build_sys_user_route() -> Router<Arc<AppState>> {
    routes! {
        "/system/user/addUser" => post(sys_user_handler::add_sys_user),
        "/system/user/deleteUser"=>post(sys_user_handler::delete_sys_user),
        "/system/user/updateUser"=>post(sys_user_handler::update_sys_user),
        "/system/user/updateUserStatus"=>post(sys_user_handler::update_sys_user_status),
        "/system/user/reset_sys_user_password"=>post(sys_user_handler::reset_sys_user_password),
        "/system/user/queryUserDetail"=>post(sys_user_handler::query_sys_user_detail),
        "/system/user/queryUserList"=>post(sys_user_handler::query_sys_user_list),
        "/system/user/login"=>post(sys_user_handler::login),
        "/system/user/queryUserMenu"=>get(sys_user_handler::query_user_menu),
        "/system/user/queryUserRole"=>post(sys_user_handler::query_user_role),
        "/system/user/updateUserRole"=>post(sys_user_handler::update_user_role),
        "/system/user/updateUserPassword"=>post(sys_user_handler::update_sys_user_password),
    }
}
