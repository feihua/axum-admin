use crate::service::system::sys_role_service::RoleService;
use crate::vo::system::sys_role_vo::*;
use crate::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use log::info;
use std::sync::Arc;
/*
 *添加角色信息
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn add_sys_role(State(state): State<Arc<AppState>>, Json(item): Json<RoleReq>) -> impl IntoResponse {
    info!("add sys_role params: {:?}", &item);

    RoleService::add_sys_role(state, item).await
}

/*
 *删除角色信息
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn delete_sys_role(State(state): State<Arc<AppState>>, Json(item): Json<DeleteRoleReq>) -> impl IntoResponse {
    info!("delete sys_role params: {:?}", &item);

    RoleService::delete_sys_role(state, item).await
}

/*
 *更新角色信息
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn update_sys_role(State(state): State<Arc<AppState>>, Json(item): Json<RoleReq>) -> impl IntoResponse {
    info!("update sys_role params: {:?}", &item);

    RoleService::update_sys_role(state, item).await
}

/*
 *更新角色信息状态
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn update_sys_role_status(State(state): State<Arc<AppState>>, Json(item): Json<UpdateRoleStatusReq>) -> impl IntoResponse {
    info!("update sys_role_status params: {:?}", &item);

    RoleService::update_sys_role_status(state, item).await
}

/*
 *查询角色信息详情
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn query_sys_role_detail(State(state): State<Arc<AppState>>, Json(item): Json<QueryRoleDetailReq>) -> impl IntoResponse {
    info!("query sys_role_detail params: {:?}", &item);

    RoleService::query_sys_role_detail(state, item).await
}

/*
 *查询角色信息列表
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn query_sys_role_list(State(state): State<Arc<AppState>>, Json(item): Json<QueryRoleListReq>) -> impl IntoResponse {
    info!("query sys_role_list params: {:?}", &item);

    RoleService::query_sys_role_list(state, item).await
}

/*
 *查询角色关联的菜单
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn query_role_menu(State(state): State<Arc<AppState>>, Json(item): Json<QueryRoleMenuReq>) -> impl IntoResponse {
    info!("query role_menu params: {:?}", &item);

    RoleService::query_role_menu(state, item).await
}

/*
 *更新角色关联的菜单
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn update_role_menu(State(state): State<Arc<AppState>>, Json(item): Json<UpdateRoleMenuReq>) -> impl IntoResponse {
    info!("update role_menu params: {:?}", &item);

    RoleService::update_role_menu(state, item).await
}

/*
 *查询已分配用户角色列表
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn query_allocated_list(State(state): State<Arc<AppState>>, Json(item): Json<AllocatedListReq>) -> impl IntoResponse {
    info!("query_allocated_list params: {:?}", &item);

    RoleService::query_allocated_list(state, item).await
}

/*
 *查询未分配用户角色列表
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn query_unallocated_list(State(state): State<Arc<AppState>>, Json(item): Json<UnallocatedListReq>) -> impl IntoResponse {
    info!("query_unallocated_list params: {:?}", &item);

    RoleService::query_unallocated_list(state, item).await
}

/*
 *取消授权用户
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn cancel_auth_user(State(state): State<Arc<AppState>>, Json(item): Json<CancelAuthUserReq>) -> impl IntoResponse {
    info!("cancel_auth_user params: {:?}", &item);

    RoleService::cancel_auth_user(state, item).await
}

/*
 *批量取消授权用户
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn batch_cancel_auth_user(State(state): State<Arc<AppState>>, Json(item): Json<CancelAuthUserAllReq>) -> impl IntoResponse {
    info!("cancel auth_user_all params: {:?}", &item);

    RoleService::batch_cancel_auth_user(state, item).await
}

/*
 *批量选择用户授权
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn batch_auth_user(State(state): State<Arc<AppState>>, Json(item): Json<SelectAuthUserAllReq>) -> impl IntoResponse {
    info!("batch_auth_user params: {:?}", &item);

    RoleService::batch_auth_user(state, item).await
}
