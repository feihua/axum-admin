use crate::AppState;
use crate::handler::system::sys_dict_type_handler;
use axum::Router;
use route_macro::routes;
use std::sync::Arc;
/*
 *构建字典类型表路由
 *author：刘飞华
 *date：2024/12/25 14:07:37
 */
pub fn build_sys_dict_type_route() -> Router<Arc<AppState>> {
    routes! {
        "/system/dictType/addDictType"=>post(sys_dict_type_handler::add_sys_dict_type),
        "/system/dictType/deleteDictType"=>post(sys_dict_type_handler::delete_sys_dict_type),
        "/system/dictType/updateDictType"=>post(sys_dict_type_handler::update_sys_dict_type),
        "/system/dictType/updateDictTypeStatus"=>post(sys_dict_type_handler::update_sys_dict_type_status),
        "/system/dictType/queryDictTypeDetail"=>post(sys_dict_type_handler::query_sys_dict_type_detail),
        "/system/dictType/queryDictTypeList"=>post(sys_dict_type_handler::query_sys_dict_type_list),
    }
}
