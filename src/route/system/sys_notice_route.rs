use crate::handler::system::sys_notice_handler;
use crate::AppState;
use axum::routing::post;
use axum::Router;
use std::sync::Arc;
/*
 *构建通知公告表路由
 *author：刘飞华
 *date：2024/12/25 14:07:37
 */
pub fn build_sys_notice_route() -> Router<Arc<AppState>> {
    Router::new()
        .route("/system/notice/addNotice", post(sys_notice_handler::add_sys_notice))
        .route("/system/notice/deleteNotice", post(sys_notice_handler::delete_sys_notice))
        .route("/system/notice/updateNotice", post(sys_notice_handler::update_sys_notice))
        .route("/system/notice/updateNoticeStatus", post(sys_notice_handler::update_sys_notice_status))
        .route("/system/notice/queryNoticeDetail", post(sys_notice_handler::query_sys_notice_detail))
        .route("/system/notice/queryNoticeList", post(sys_notice_handler::query_sys_notice_list))
    //记得在main.rs中添加路由build_sys_notice_route()
}
