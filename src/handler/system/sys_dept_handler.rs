use crate::common::result::ok_result_data;
use crate::service::system::sys_dept_service::DeptService;
use crate::vo::system::sys_dept_vo::*;
use crate::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use axum_valid::Valid;
use log::info;
use std::sync::Arc;
/*
 *添加部门表
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn add_sys_dept(State(state): State<Arc<AppState>>, Valid(Json(item)): Valid<Json<DeptReq>>) -> impl IntoResponse {
    info!("add sys_dept params: {:?}", &item);

    DeptService::add_sys_dept(state, item).await.map(|last_insert_id| ok_result_data(last_insert_id))?
}

/*
 *删除部门表
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn delete_sys_dept(State(state): State<Arc<AppState>>, Valid(Json(item)): Valid<Json<DeleteDeptReq>>) -> impl IntoResponse {
    info!("delete sys_dept params: {:?}", &item);

    DeptService::delete_sys_dept(state, item).await.map(|rows_affected| ok_result_data(rows_affected))?
}

/*
 *更新部门表
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn update_sys_dept(State(state): State<Arc<AppState>>, Valid(Json(item)): Valid<Json<DeptReq>>) -> impl IntoResponse {
    info!("update sys_dept params: {:?}", &item);

    DeptService::update_sys_dept(state, item).await.map(|rows_affected| ok_result_data(rows_affected))?
}

/*
 *更新部门表状态
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn update_sys_dept_status(State(state): State<Arc<AppState>>, Valid(Json(item)): Valid<Json<UpdateDeptStatusReq>>) -> impl IntoResponse {
    info!("update sys_dept_status params: {:?}", &item);

    DeptService::update_sys_dept_status(state, item).await.map(|rows_affected| ok_result_data(rows_affected))?
}
/*
 *查询部门表详情
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn query_sys_dept_detail(State(state): State<Arc<AppState>>, Valid(Json(item)): Valid<Json<QueryDeptDetailReq>>) -> impl IntoResponse {
    info!("query sys_dept_detail params: {:?}", &item);

    DeptService::query_sys_dept_detail(state, item).await.map(|record| ok_result_data(record))?
}

/*
 *查询部门表列表
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn query_sys_dept_list(State(state): State<Arc<AppState>>, Valid(Json(item)): Valid<Json<QueryDeptListReq>>) -> impl IntoResponse {
    info!("query sys_dept_list params: {:?}", &item);

    DeptService::query_sys_dept_list(state, item).await.map(|record| ok_result_data(record))?
}
