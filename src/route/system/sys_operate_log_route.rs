use crate::handler::system::sys_operate_log_handler;
use crate::AppState;
use axum::routing::post;
use axum::Router;
use std::sync::Arc;
/*
 *构建操作日志记录路由
 *author：刘飞华
 *date：2024/12/25 14:07:37
 */
pub fn build_sys_operate_log_route() -> Router<Arc<AppState>> {
    Router::new()
        .route(
            "/system/operateLog/deleteOperateLog",
            post(sys_operate_log_handler::delete_sys_operate_log),
        )
        .route(
            "/system/operateLog/queryOperateLogDetail",
            post(sys_operate_log_handler::query_sys_operate_log_detail),
        )
        .route(
            "/system/operateLog/queryOperateLogList",
            post(sys_operate_log_handler::query_sys_operate_log_list),
        )
    //记得在main.rs中添加路由build_sys_operate_log_route()
}
