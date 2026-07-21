use crate::service::system::sys_operate_log_service::OperateLogService;
use crate::vo::system::sys_operate_log_vo::*;
use crate::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use log::info;
use std::sync::Arc;
/*
 *删除操作日志记录
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn delete_sys_operate_log(State(state): State<Arc<AppState>>, Json(item): Json<DeleteOperateLogReq>) -> impl IntoResponse {
    info!("delete sys_operate_log params: {:?}", &item);

    OperateLogService::delete_sys_operate_log(state, item).await
}

/*
 *清空操作日志记录
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn clean_sys_operate_log(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    info!("clean sys_operate_log");

    OperateLogService::clean_sys_operate_log(state).await
}

/*
 *查询操作日志记录详情
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn query_sys_operate_log_detail(State(state): State<Arc<AppState>>, Json(item): Json<QueryOperateLogDetailReq>) -> impl IntoResponse {
    info!("query sys_operate_log_detail params: {:?}", &item);

    OperateLogService::query_sys_operate_log_detail(state, item).await
}
/*
 *查询操作日志记录列表
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn query_sys_operate_log_list(State(state): State<Arc<AppState>>, Json(item): Json<QueryOperateLogListReq>) -> impl IntoResponse {
    info!("query sys_operate_log_list params: {:?}", &item);

    OperateLogService::query_sys_operate_log_list(state, item).await
}
