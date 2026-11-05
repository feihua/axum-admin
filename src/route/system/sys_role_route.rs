use crate::AppState;
use crate::handler::system::sys_role_handler;
use axum::Router;
use route_macro::routes;
use std::sync::Arc;
/*
 *构建角色信息路由
 *author：刘飞华
 *date：2024/12/12 17:04:49
 */
pub fn build_sys_role_route() -> Router<Arc<AppState>> {
    routes! {
        "/system/role/addRole"=>post(sys_role_handler::add_sys_role),
        "/system/role/deleteRole"=>post(sys_role_handler::delete_sys_role),
        "/system/role/updateRole"=>post(sys_role_handler::update_sys_role),
        "/system/role/updateRoleStatus"=>post(sys_role_handler::update_sys_role_status),
        "/system/role/queryRoleDetail"=>post(sys_role_handler::query_sys_role_detail),
        "/system/role/queryRoleList"=>post(sys_role_handler::query_sys_role_list),
        "/system/role/queryRoleMenu"=>post(sys_role_handler::query_role_menu),
        "/system/role/updateRoleMenu"=>post(sys_role_handler::update_role_menu),
        "/system/role/queryAllocatedList"=>post(sys_role_handler::query_allocated_list),
        "/system/role/queryUnallocatedList"=>post(sys_role_handler::query_unallocated_list),
        "/system/role/cancelAuthUser"=>post(sys_role_handler::cancel_auth_user),
        "/system/role/batchCancelAuthUser"=>post(sys_role_handler::batch_cancel_auth_user),
        "/system/role/batchAuthUser"=>post(sys_role_handler::batch_auth_user),
    }
}
