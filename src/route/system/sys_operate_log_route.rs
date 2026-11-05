use crate::AppState;
use crate::handler::system::sys_operate_log_handler;
use axum::Router;
use route_macro::routes;
use std::sync::Arc;
/*
 *构建操作日志记录路由
 *author：刘飞华
 *date：2024/12/25 14:07:37
 */
pub fn build_sys_operate_log_route() -> Router<Arc<AppState>> {
    routes! {
        "/system/operateLog/deleteOperateLog"=>post(sys_operate_log_handler::delete_sys_operate_log),
        "/system/operateLog/cleanOperateLog"=>get(sys_operate_log_handler::clean_sys_operate_log),
        "/system/operateLog/queryOperateLogDetail"=>post(sys_operate_log_handler::query_sys_operate_log_detail),
        "/system/operateLog/queryOperateLogList"=>post(sys_operate_log_handler::query_sys_operate_log_list),
    }
}
