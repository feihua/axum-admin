use crate::service::system::sys_dict_type_service::DictTypeService;
use crate::vo::system::sys_dict_type_vo::*;
use crate::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use log::info;
use std::sync::Arc;
/*
 *添加字典类型
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn add_sys_dict_type(State(state): State<Arc<AppState>>, Json(item): Json<DictTypeReq>) -> impl IntoResponse {
    info!("add sys_dict_type params: {:?}", &item);

    DictTypeService::add_sys_dict_type(state, item).await
}

/*
 *删除字典类型
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn delete_sys_dict_type(State(state): State<Arc<AppState>>, Json(item): Json<DeleteDictTypeReq>) -> impl IntoResponse {
    info!("delete sys_dict_type params: {:?}", &item);

    DictTypeService::delete_sys_dict_type(state, item).await
}

/*
 *更新字典类型
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn update_sys_dict_type(State(state): State<Arc<AppState>>, Json(item): Json<DictTypeReq>) -> impl IntoResponse {
    info!("update sys_dict_type params: {:?}", &item);

    DictTypeService::update_sys_dict_type(state, item).await
}

/*
 *更新字典类型状态
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn update_sys_dict_type_status(State(state): State<Arc<AppState>>, Json(item): Json<UpdateDictTypeStatusReq>) -> impl IntoResponse {
    info!("update sys_dict_type_status params: {:?}", &item);

    DictTypeService::update_sys_dict_type_status(state, item).await
}

/*
 *查询字典类型详情
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn query_sys_dict_type_detail(State(state): State<Arc<AppState>>, Json(item): Json<QueryDictTypeDetailReq>) -> impl IntoResponse {
    info!("query sys_dict_type_detail params: {:?}", &item);

    DictTypeService::query_sys_dict_type_detail(state, item).await
}

/*
 *查询字典类型列
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn query_sys_dict_type_list(State(state): State<Arc<AppState>>, Json(item): Json<QueryDictTypeListReq>) -> impl IntoResponse {
    info!("query sys_dict_type_list params: {:?}", &item);

    DictTypeService::query_sys_dict_type_list(state, item).await
}
