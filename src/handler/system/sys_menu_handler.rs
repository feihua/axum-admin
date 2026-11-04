use crate::AppState;
use crate::common::result::{ok_result_data, ok_result_page};
use crate::service::system::sys_menu_service::MenuService;
use crate::vo::system::sys_menu_vo::*;
use axum::Json;
use axum::extract::State;
use axum::response::IntoResponse;
use axum_valid::Valid;
use std::sync::Arc;

use print_log_macro::log;
/*
 *添加菜单信息
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
#[log]
pub async fn add_sys_menu(State(state): State<Arc<AppState>>, Valid(Json(item)): Valid<Json<MenuReq>>) -> impl IntoResponse {
    MenuService::add_sys_menu(state, item).await.map(|last_insert_id| ok_result_data(last_insert_id))?
}

/*
 *删除菜单信息
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
#[log]
pub async fn delete_sys_menu(State(state): State<Arc<AppState>>, Valid(Json(item)): Valid<Json<DeleteMenuReq>>) -> impl IntoResponse {
    MenuService::delete_sys_menu(state, item).await.map(|rows_affected| ok_result_data(rows_affected))?
}

/*
 *更新菜单信息
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
#[log]
pub async fn update_sys_menu(State(state): State<Arc<AppState>>, Valid(Json(item)): Valid<Json<MenuReq>>) -> impl IntoResponse {
    MenuService::update_sys_menu(state, item).await.map(|rows_affected| ok_result_data(rows_affected))?
}

/*
 *更新菜单信息状态
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
#[log]
pub async fn update_sys_menu_status(State(state): State<Arc<AppState>>, Valid(Json(item)): Valid<Json<UpdateMenuStatusReq>>) -> impl IntoResponse {
    MenuService::update_sys_menu_status(state, item).await.map(|rows_affected| ok_result_data(rows_affected))?
}

/*
 *查询菜单信息详情
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
#[log]
pub async fn query_sys_menu_detail(State(state): State<Arc<AppState>>, Valid(Json(item)): Valid<Json<QueryMenuDetailReq>>) -> impl IntoResponse {
    MenuService::query_sys_menu_detail(state, item).await.map(|record| ok_result_data(record))?
}

/*
 *查询菜单信息列表
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
#[log]
pub async fn query_sys_menu_list(State(state): State<Arc<AppState>>, Valid(Json(item)): Valid<Json<QueryMenuListReq>>) -> impl IntoResponse {
    MenuService::query_sys_menu_list(state).await.map(|record| ok_result_data(record))?
}

/*
 *查询菜单信息(排除按钮)
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
#[log]
pub async fn query_sys_menu_list_simple(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    MenuService::query_sys_menu_list_simple(state).await.map(|record| ok_result_data(record))?
}

/*
 *查询菜单信息列表
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
#[log]
pub async fn query_sys_menu_resource_list(State(state): State<Arc<AppState>>, Valid(Json(item)): Valid<Json<QueryMenuListReq>>) -> impl IntoResponse {
    MenuService::query_sys_menu_resource_list(state, item).await.map(|(list, total)| ok_result_page(list, total))?
}
