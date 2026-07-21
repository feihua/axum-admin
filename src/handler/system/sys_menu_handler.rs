use crate::service::system::sys_menu_service::MenuService;
use crate::vo::system::sys_menu_vo::*;
use crate::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use log::info;
use std::sync::Arc;
/*
 *添加菜单信息
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn add_sys_menu(State(state): State<Arc<AppState>>, Json(item): Json<MenuReq>) -> impl IntoResponse {
    info!("add sys_menu params: {:?}", &item);

    MenuService::add_sys_menu(state, item).await
}

/*
 *删除菜单信息
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn delete_sys_menu(State(state): State<Arc<AppState>>, Json(item): Json<DeleteMenuReq>) -> impl IntoResponse {
    info!("delete sys_menu params: {:?}", &item);

    MenuService::delete_sys_menu(state, item).await
}

/*
 *更新菜单信息
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn update_sys_menu(State(state): State<Arc<AppState>>, Json(item): Json<MenuReq>) -> impl IntoResponse {
    info!("update sys_menu params: {:?}", &item);

    MenuService::update_sys_menu(state, item).await
}

/*
 *更新菜单信息状态
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn update_sys_menu_status(State(state): State<Arc<AppState>>, Json(item): Json<UpdateMenuStatusReq>) -> impl IntoResponse {
    info!("update sys_menu_status params: {:?}", &item);

    MenuService::update_sys_menu_status(state, item).await
}

/*
 *查询菜单信息详情
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn query_sys_menu_detail(State(state): State<Arc<AppState>>, Json(item): Json<QueryMenuDetailReq>) -> impl IntoResponse {
    info!("query sys_menu_detail params: {:?}", &item);

    MenuService::query_sys_menu_detail(state, item).await
}

/*
 *查询菜单信息列表
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn query_sys_menu_list(State(state): State<Arc<AppState>>, Json(item): Json<QueryMenuListReq>) -> impl IntoResponse {
    info!("query sys_menu_list params: {:?}", &item);

    MenuService::query_sys_menu_list(state).await
}

/*
 *查询菜单信息(排除按钮)
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn query_sys_menu_list_simple(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    MenuService::query_sys_menu_list_simple(state).await
}

/*
 *查询菜单信息列表
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn query_sys_menu_resource_list(State(state): State<Arc<AppState>>, Json(req): Json<QueryMenuListReq>) -> impl IntoResponse {
    info!("query sys_menu_list params: {:?}", &req);

    MenuService::query_sys_menu_resource_list(state, req).await
}
