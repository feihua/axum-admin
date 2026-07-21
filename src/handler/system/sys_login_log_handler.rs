use crate::service::system::sys_login_log_service::LoginLogService;
use crate::vo::system::sys_login_log_vo::*;
use crate::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use log::info;
use std::sync::Arc;
/*
 *删除系统访问记录
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn delete_sys_login_log(State(state): State<Arc<AppState>>, Json(item): Json<DeleteLoginLogReq>) -> impl IntoResponse {
    info!("delete sys_login_log params: {:?}", &item);

    LoginLogService::delete_sys_login_log(state, item).await
}

/*
 *清空系统登录日志
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn clean_sys_login_log(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    info!("clean sys_login_log ");

    LoginLogService::clean_sys_login_log(state).await
}

/*
 *查询系统访问记录详情
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn query_sys_login_log_detail(State(state): State<Arc<AppState>>, Json(item): Json<QueryLoginLogDetailReq>) -> impl IntoResponse {
    info!("query sys_login_log_detail params: {:?}", &item);

    LoginLogService::query_sys_login_log_detail(state, item).await
}

/*
 *查询系统访问记录列表
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn query_sys_login_log_list(State(state): State<Arc<AppState>>, Json(item): Json<QueryLoginLogListReq>) -> impl IntoResponse {
    info!("query sys_login_log_list params: {:?}", &item);

    LoginLogService::query_sys_login_log_list(state, item).await
}
