use crate::AppState;
use axum::routing::post;
use axum::Router;
use std::sync::Arc;

/*
 *构建操作日志记录路由
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
pub fn build_sys_operate_log_route() -> Router<Arc<AppState>> {
    Router::new()
        .route(
            "/addOperateLog",
            post(sys_operate_log_handler::add_sys_operate_log),
        )
        .route(
            "/deleteOperateLog",
            post(sys_operate_log_handler::delete_sys_operate_log),
        )
        .route(
            "/updateOperateLog",
            post(sys_operate_log_handler::update_sys_operate_log),
        )
        .route(
            "/updateOperateLogStatus",
            post(sys_operate_log_handler::update_sys_operate_log_status),
        )
        .route(
            "/queryOperateLogDetail",
            post(sys_operate_log_handler::query_sys_operate_log_detail),
        )
        .route(
            "/queryOperateLogList",
            post(sys_operate_log_handler::query_sys_operate_log_list),
        )
    //记得在main.rs中添加路由build_sys_operate_log_route()
}
