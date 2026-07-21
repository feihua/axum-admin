use crate::service::system::sys_dict_data_service::DictDataService;
use crate::vo::system::sys_dict_data_vo::*;
use crate::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use log::info;
use std::sync::Arc;
/*
 *添加字典数据
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn add_sys_dict_data(State(state): State<Arc<AppState>>, Json(item): Json<DictDataReq>) -> impl IntoResponse {
    info!("add sys_dict_data params: {:?}", &item);

    DictDataService::add_sys_dict_data(state, item).await
}

/*
 *删除字典数据
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn delete_sys_dict_data(State(state): State<Arc<AppState>>, Json(item): Json<DeleteDictDataReq>) -> impl IntoResponse {
    info!("delete sys_dict_data params: {:?}", &item);

    DictDataService::delete_sys_dict_data(state, item).await
}

/*
 *更新字典数据
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn update_sys_dict_data(State(state): State<Arc<AppState>>, Json(item): Json<DictDataReq>) -> impl IntoResponse {
    info!("update sys_dict_data params: {:?}", &item);

    DictDataService::update_sys_dict_data(state, item).await
}

/*
 *更新字典数据状态
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn update_sys_dict_data_status(State(state): State<Arc<AppState>>, Json(item): Json<UpdateDictDataStatusReq>) -> impl IntoResponse {
    info!("update sys_dict_data_status params: {:?}", &item);

    DictDataService::update_sys_dict_data_status(state, item).await
}

/*
 *查询字典数据详情
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn query_sys_dict_data_detail(State(state): State<Arc<AppState>>, Json(item): Json<QueryDictDataDetailReq>) -> impl IntoResponse {
    info!("query sys_dict_data_detail params: {:?}", &item);

    DictDataService::query_sys_dict_data_detail(state, item).await
}

/*
 *查询字典数据列
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn query_sys_dict_data_list(State(state): State<Arc<AppState>>, Json(item): Json<QueryDictDataListReq>) -> impl IntoResponse {
    info!("query sys_dict_data_list params: {:?}", &item);

    DictDataService::query_sys_dict_data_list(state, item).await
}
