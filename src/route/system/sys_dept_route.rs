use crate::AppState;
use axum::routing::post;
use axum::Router;
use std::sync::Arc;

/*
 *构建部门表路由
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
pub fn build_sys_dept_route() -> Router<Arc<AppState>> {
    Router::new()
        .route("/addDept", post(sys_dept_handler::add_sys_dept))
        .route("/deleteDept", post(sys_dept_handler::delete_sys_dept))
        .route("/updateDept", post(sys_dept_handler::update_sys_dept))
        .route(
            "/updateDeptStatus",
            post(sys_dept_handler::update_sys_dept_status),
        )
        .route(
            "/queryDeptDetail",
            post(sys_dept_handler::query_sys_dept_detail),
        )
        .route(
            "/queryDeptList",
            post(sys_dept_handler::query_sys_dept_list),
        )
    //记得在main.rs中添加路由build_sys_dept_route()
}
