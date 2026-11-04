use crate::AppState;
use crate::common::result::ok_result_data;
use crate::service::system::sys_dept_service::DeptService;
use crate::vo::system::sys_dept_vo::*;
use axum::Json;
use axum::extract::State;
use axum::response::IntoResponse;
use axum_valid::Valid;
use std::sync::Arc;

use print_log_macro::log;
/*
 *添加部门表
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
#[log]
pub async fn add_sys_dept(State(state): State<Arc<AppState>>, Valid(Json(item)): Valid<Json<DeptReq>>) -> impl IntoResponse {
    DeptService::add_sys_dept(state, item).await.map(|last_insert_id| ok_result_data(last_insert_id))?
}

/*
 *删除部门表
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
#[log]
pub async fn delete_sys_dept(State(state): State<Arc<AppState>>, Valid(Json(item)): Valid<Json<DeleteDeptReq>>) -> impl IntoResponse {
    DeptService::delete_sys_dept(state, item).await.map(|rows_affected| ok_result_data(rows_affected))?
}

/*
 *更新部门表
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
#[log]
pub async fn update_sys_dept(State(state): State<Arc<AppState>>, Valid(Json(item)): Valid<Json<DeptReq>>) -> impl IntoResponse {
    DeptService::update_sys_dept(state, item).await.map(|rows_affected| ok_result_data(rows_affected))?
}

/*
 *更新部门表状态
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
#[log]
pub async fn update_sys_dept_status(State(state): State<Arc<AppState>>, Valid(Json(item)): Valid<Json<UpdateDeptStatusReq>>) -> impl IntoResponse {
    DeptService::update_sys_dept_status(state, item).await.map(|rows_affected| ok_result_data(rows_affected))?
}
/*
 *查询部门表详情
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
#[log]
pub async fn query_sys_dept_detail(State(state): State<Arc<AppState>>, Valid(Json(item)): Valid<Json<QueryDeptDetailReq>>) -> impl IntoResponse {
    DeptService::query_sys_dept_detail(state, item).await.map(|record| ok_result_data(record))?
}

/*
 *查询部门表列表
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
#[log]
pub async fn query_sys_dept_list(State(state): State<Arc<AppState>>, Valid(Json(item)): Valid<Json<QueryDeptListReq>>) -> impl IntoResponse {
    DeptService::query_sys_dept_list(state, item).await.map(|record| ok_result_data(record))?
}
