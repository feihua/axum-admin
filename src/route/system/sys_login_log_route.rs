use crate::AppState;
use axum::routing::post;
use axum::Router;
use std::sync::Arc;

/*
 *构建系统访问记录路由
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
pub fn build_sys_login_log_route() -> Router<Arc<AppState>> {
    Router::new()
        .route(
            "/addLoginLog",
            post(sys_login_log_handler::add_sys_login_log),
        )
        .route(
            "/deleteLoginLog",
            post(sys_login_log_handler::delete_sys_login_log),
        )
        .route(
            "/updateLoginLog",
            post(sys_login_log_handler::update_sys_login_log),
        )
        .route(
            "/updateLoginLogStatus",
            post(sys_login_log_handler::update_sys_login_log_status),
        )
        .route(
            "/queryLoginLogDetail",
            post(sys_login_log_handler::query_sys_login_log_detail),
        )
        .route(
            "/queryLoginLogList",
            post(sys_login_log_handler::query_sys_login_log_list),
        )
    //记得在main.rs中添加路由build_sys_login_log_route()
}
