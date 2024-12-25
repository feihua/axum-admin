use crate::AppState;
use axum::routing::post;
use axum::Router;
use std::sync::Arc;

/*
 *构建字典数据表路由
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
pub fn build_sys_dict_data_route() -> Router<Arc<AppState>> {
    Router::new()
        .route(
            "/addDictData",
            post(sys_dict_data_handler::add_sys_dict_data),
        )
        .route(
            "/deleteDictData",
            post(sys_dict_data_handler::delete_sys_dict_data),
        )
        .route(
            "/updateDictData",
            post(sys_dict_data_handler::update_sys_dict_data),
        )
        .route(
            "/updateDictDataStatus",
            post(sys_dict_data_handler::update_sys_dict_data_status),
        )
        .route(
            "/queryDictDataDetail",
            post(sys_dict_data_handler::query_sys_dict_data_detail),
        )
        .route(
            "/queryDictDataList",
            post(sys_dict_data_handler::query_sys_dict_data_list),
        )
    //记得在main.rs中添加路由build_sys_dict_data_route()
}
