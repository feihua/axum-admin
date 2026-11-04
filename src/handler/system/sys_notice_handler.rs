use crate::AppState;
use crate::common::result::{ok_result_data, ok_result_page};
use crate::service::system::sys_notice_service::NoticeService;
use crate::vo::system::sys_notice_vo::*;
use axum::Json;
use axum::extract::State;
use axum::response::IntoResponse;
use axum_valid::Valid;
use std::sync::Arc;

use print_log_macro::log;
/*
 *添加通知公告表
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
#[log]
pub async fn add_sys_notice(State(state): State<Arc<AppState>>, Valid(Json(item)): Valid<Json<NoticeReq>>) -> impl IntoResponse {
    NoticeService::add_sys_notice(state, item).await.map(|last_insert_id| ok_result_data(last_insert_id))?
}

/*
 *删除通知公告表
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
#[log]
pub async fn delete_sys_notice(State(state): State<Arc<AppState>>, Valid(Json(item)): Valid<Json<DeleteNoticeReq>>) -> impl IntoResponse {
    NoticeService::delete_sys_notice(state, item).await.map(|rows_affected| ok_result_data(rows_affected))?
}

/*
 *更新通知公告表
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
#[log]
pub async fn update_sys_notice(State(state): State<Arc<AppState>>, Valid(Json(item)): Valid<Json<NoticeReq>>) -> impl IntoResponse {
    NoticeService::update_sys_notice(state, item).await.map(|rows_affected| ok_result_data(rows_affected))?
}

/*
 *更新通知公告表状态
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
#[log]
pub async fn update_sys_notice_status(State(state): State<Arc<AppState>>, Valid(Json(item)): Valid<Json<UpdateNoticeStatusReq>>) -> impl IntoResponse {
    NoticeService::update_sys_notice_status(state, item).await.map(|rows_affected| ok_result_data(rows_affected))?
}

/*
 *查询通知公告表详情
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
#[log]
pub async fn query_sys_notice_detail(State(state): State<Arc<AppState>>, Valid(Json(item)): Valid<Json<QueryNoticeDetailReq>>) -> impl IntoResponse {
    NoticeService::query_sys_notice_detail(state, item).await.map(|record| ok_result_data(record))?
}

/*
 *查询通知公告表列表
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
#[log]
pub async fn query_sys_notice_list(State(state): State<Arc<AppState>>, Valid(Json(item)): Valid<Json<QueryNoticeListReq>>) -> impl IntoResponse {
    NoticeService::query_sys_notice_list(state, item).await.map(|(list, total)| ok_result_page(list, total))?
}
