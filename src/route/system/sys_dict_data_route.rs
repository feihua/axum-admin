use crate::handler::system::sys_dict_data_handler;
use crate::AppState;
use axum::routing::post;
use axum::Router;
use std::sync::Arc;
/*
 *构建字典数据表路由
 *author：刘飞华
 *date：2024/12/25 14:07:37
 */
pub fn build_sys_dict_data_route() -> Router<Arc<AppState>> {
    Router::new()
        .route(
            "/system/dictData/addDictData",
            post(sys_dict_data_handler::add_sys_dict_data),
        )
        .route(
            "/system/dictData/deleteDictData",
            post(sys_dict_data_handler::delete_sys_dict_data),
        )
        .route(
            "/system/dictData/updateDictData",
            post(sys_dict_data_handler::update_sys_dict_data),
        )
        .route(
            "/system/dictData/updateDictDataStatus",
            post(sys_dict_data_handler::update_sys_dict_data_status),
        )
        .route(
            "/system/dictData/queryDictDataDetail",
            post(sys_dict_data_handler::query_sys_dict_data_detail),
        )
        .route(
            "/system/dictData/queryDictDataList",
            post(sys_dict_data_handler::query_sys_dict_data_list),
        )
    //记得在main.rs中添加路由build_sys_dict_data_route()
}
