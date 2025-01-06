use crate::handler::system::sys_role_handler;
use crate::AppState;
use axum::routing::post;
use axum::Router;
use std::sync::Arc;

/*
 *构建角色信息路由
 *author：刘飞华
 *date：2024/12/12 17:04:49
 */
pub fn build_sys_role_route() -> Router<Arc<AppState>> {
    Router::new()
        .route("/system/role/addRole", post(sys_role_handler::add_sys_role))
        .route(
            "/system/role/deleteRole",
            post(sys_role_handler::delete_sys_role),
        )
        .route(
            "/system/role/updateRole",
            post(sys_role_handler::update_sys_role),
        )
        .route(
            "/system/role/updateRoleStatus",
            post(sys_role_handler::update_sys_role_status),
        )
        .route(
            "/system/role/queryRoleDetail",
            post(sys_role_handler::query_sys_role_detail),
        )
        .route(
            "/system/role/queryRoleList",
            post(sys_role_handler::query_sys_role_list),
        )
        .route(
            "/system/role/queryRoleMenu",
            post(sys_role_handler::query_role_menu),
        )
        .route(
            "/system/role/updateRoleMenu",
            post(sys_role_handler::update_role_menu),
        )
        .route(
            "/system/role/queryAllocatedList",
            post(sys_role_handler::query_allocated_list),
        )
        .route(
            "/system/role/queryUnallocatedList",
            post(sys_role_handler::query_unallocated_list),
        )
        .route(
            "/system/role/cancelAuthUser",
            post(sys_role_handler::cancel_auth_user),
        )
        .route(
            "/system/role/batchCancelAuthUser",
            post(sys_role_handler::batch_cancel_auth_user),
        )
        .route(
            "/system/role/batchAuthUser",
            post(sys_role_handler::batch_auth_user),
        )
    //记得在main.rs中添加路由build_sys_role_route()
}
