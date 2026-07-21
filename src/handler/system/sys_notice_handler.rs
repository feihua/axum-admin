use crate::service::system::sys_notice_service::NoticeService;
use crate::vo::system::sys_notice_vo::*;
use crate::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use log::info;
use std::sync::Arc;
/*
 *添加通知公告表
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn add_sys_notice(State(state): State<Arc<AppState>>, Json(item): Json<NoticeReq>) -> impl IntoResponse {
    info!("add sys_notice params: {:?}", &item);

    NoticeService::add_sys_notice(state, item).await
}

/*
 *删除通知公告表
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn delete_sys_notice(State(state): State<Arc<AppState>>, Json(item): Json<DeleteNoticeReq>) -> impl IntoResponse {
    info!("delete sys_notice params: {:?}", &item);

    NoticeService::delete_sys_notice(state, item).await
}

/*
 *更新通知公告表
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn update_sys_notice(State(state): State<Arc<AppState>>, Json(item): Json<NoticeReq>) -> impl IntoResponse {
    info!("update sys_notice params: {:?}", &item);

    NoticeService::update_sys_notice(state, item).await
}

/*
 *更新通知公告表状态
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn update_sys_notice_status(State(state): State<Arc<AppState>>, Json(item): Json<UpdateNoticeStatusReq>) -> impl IntoResponse {
    info!("update sys_notice_status params: {:?}", &item);

    NoticeService::update_sys_notice_status(state, item).await
}

/*
 *查询通知公告表详情
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn query_sys_notice_detail(State(state): State<Arc<AppState>>, Json(item): Json<QueryNoticeDetailReq>) -> impl IntoResponse {
    info!("query sys_notice_detail params: {:?}", &item);

    NoticeService::query_sys_notice_detail(state, item).await
}

/*
 *查询通知公告表列表
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn query_sys_notice_list(State(state): State<Arc<AppState>>, Json(item): Json<QueryNoticeListReq>) -> impl IntoResponse {
    info!("query sys_notice_list params: {:?}", &item);

    NoticeService::query_sys_notice_list(state, item).await
}
