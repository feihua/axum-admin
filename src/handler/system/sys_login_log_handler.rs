use crate::AppState;
use crate::common::result::{ok_result_data, ok_result_page};
use crate::service::system::sys_login_log_service::LoginLogService;
use crate::vo::system::sys_login_log_vo::*;
use axum::Json;
use axum::extract::State;
use axum::response::IntoResponse;
use axum_valid::Valid;
use std::sync::Arc;

use print_log_macro::log;
/*
 *删除系统访问记录
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
#[log]
pub async fn delete_sys_login_log(State(state): State<Arc<AppState>>, Valid(Json(item)): Valid<Json<DeleteLoginLogReq>>) -> impl IntoResponse {
    LoginLogService::delete_sys_login_log(state, item).await.map(|rows_affected| ok_result_data(rows_affected))?
}

/*
 *清空系统登录日志
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
#[log]
pub async fn clean_sys_login_log(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    LoginLogService::clean_sys_login_log(state).await.map(|rows_affected| ok_result_data(rows_affected))?
}

/*
 *查询系统访问记录详情
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
#[log]
pub async fn query_sys_login_log_detail(State(state): State<Arc<AppState>>, Valid(Json(item)): Valid<Json<QueryLoginLogDetailReq>>) -> impl IntoResponse {
    LoginLogService::query_sys_login_log_detail(state, item).await.map(|record| ok_result_data(record))?
}

/*
 *查询系统访问记录列表
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
#[log]
pub async fn query_sys_login_log_list(State(state): State<Arc<AppState>>, Valid(Json(item)): Valid<Json<QueryLoginLogListReq>>) -> impl IntoResponse {
    LoginLogService::query_sys_login_log_list(state, item).await.map(|(list, total)| ok_result_page(list, total))?
}
