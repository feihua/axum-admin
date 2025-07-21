use crate::handler::system::sys_dict_type_handler;
use crate::AppState;
use axum::routing::post;
use axum::Router;
use std::sync::Arc;
/*
 *构建字典类型表路由
 *author：刘飞华
 *date：2024/12/25 14:07:37
 */
pub fn build_sys_dict_type_route() -> Router<Arc<AppState>> {
    Router::new()
        .route("/system/dictType/addDictType", post(sys_dict_type_handler::add_sys_dict_type))
        .route("/system/dictType/deleteDictType", post(sys_dict_type_handler::delete_sys_dict_type))
        .route("/system/dictType/updateDictType", post(sys_dict_type_handler::update_sys_dict_type))
        .route("/system/dictType/updateDictTypeStatus", post(sys_dict_type_handler::update_sys_dict_type_status))
        .route("/system/dictType/queryDictTypeDetail", post(sys_dict_type_handler::query_sys_dict_type_detail))
        .route("/system/dictType/queryDictTypeList", post(sys_dict_type_handler::query_sys_dict_type_list))
    //记得在main.rs中添加路由build_sys_dict_type_route()
}
