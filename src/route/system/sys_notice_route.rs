use crate::AppState;
use axum::routing::post;
use axum::Router;
use std::sync::Arc;

/*
 *构建通知公告表路由
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
pub fn build_sys_notice_route() -> Router<Arc<AppState>> {
    Router::new()
        .route("/addNotice", post(sys_notice_handler::add_sys_notice))
        .route("/deleteNotice", post(sys_notice_handler::delete_sys_notice))
        .route("/updateNotice", post(sys_notice_handler::update_sys_notice))
        .route(
            "/updateNoticeStatus",
            post(sys_notice_handler::update_sys_notice_status),
        )
        .route(
            "/queryNoticeDetail",
            post(sys_notice_handler::query_sys_notice_detail),
        )
        .route(
            "/queryNoticeList",
            post(sys_notice_handler::query_sys_notice_list),
        )
    //记得在main.rs中添加路由build_sys_notice_route()
}
