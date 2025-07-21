use crate::handler::system::sys_login_log_handler;
use crate::AppState;
use axum::routing::{get, post};
use axum::Router;
use std::sync::Arc;
/*
 *构建系统访问记录路由
 *author：刘飞华
 *date：2024/12/25 14:07:37
 */
pub fn build_sys_login_log_route() -> Router<Arc<AppState>> {
    Router::new()
        .route("/system/loginLog/deleteLoginLog", post(sys_login_log_handler::delete_sys_login_log))
        .route("/system/loginLog/cleanLoginLog", get(sys_login_log_handler::clean_sys_login_log))
        .route("/system/loginLog/queryLoginLogDetail", post(sys_login_log_handler::query_sys_login_log_detail))
        .route("/system/loginLog/queryLoginLogList", post(sys_login_log_handler::query_sys_login_log_list))
    //记得在main.rs中添加路由build_sys_login_log_route()
}
