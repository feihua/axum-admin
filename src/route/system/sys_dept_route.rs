use crate::AppState;
use crate::handler::system::sys_dept_handler;
use axum::Router;
use route_macro::routes;
use std::sync::Arc;
/*
 *构建部门表路由
 *author：刘飞华
 *date：2024/12/25 14:07:37
 */
pub fn build_sys_dept_route() -> Router<Arc<AppState>> {
    routes! {
        "/system/dept/addDept"=>post(sys_dept_handler::add_sys_dept),
        "/system/dept/deleteDept"=>post(sys_dept_handler::delete_sys_dept),
        "/system/dept/updateDept"=>post(sys_dept_handler::update_sys_dept),
        "/system/dept/updateDeptStatus"=>post(sys_dept_handler::update_sys_dept_status),
        "/system/dept/queryDeptDetail"=>post(sys_dept_handler::query_sys_dept_detail),
        "/system/dept/queryDeptList"=>post(sys_dept_handler::query_sys_dept_list),
    }
}
