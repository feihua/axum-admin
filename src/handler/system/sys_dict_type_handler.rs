use crate::AppState;
use crate::common::result::{ok_result_data, ok_result_page};
use crate::service::system::sys_dict_type_service::DictTypeService;
use crate::vo::system::sys_dict_type_vo::*;
use axum::Json;
use axum::extract::State;
use axum::response::IntoResponse;
use axum_valid::Valid;
use std::sync::Arc;

use print_log_macro::log;
/*
 *添加字典类型
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
#[log]
pub async fn add_sys_dict_type(State(state): State<Arc<AppState>>, Valid(Json(item)): Valid<Json<DictTypeReq>>) -> impl IntoResponse {
    DictTypeService::add_sys_dict_type(state, item).await.map(|last_insert_id| ok_result_data(last_insert_id))?
}

/*
 *删除字典类型
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
#[log]
pub async fn delete_sys_dict_type(State(state): State<Arc<AppState>>, Valid(Json(item)): Valid<Json<DeleteDictTypeReq>>) -> impl IntoResponse {
    DictTypeService::delete_sys_dict_type(state, item).await.map(|rows_affected| ok_result_data(rows_affected))?
}

/*
 *更新字典类型
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
#[log]
pub async fn update_sys_dict_type(State(state): State<Arc<AppState>>, Valid(Json(item)): Valid<Json<DictTypeReq>>) -> impl IntoResponse {
    DictTypeService::update_sys_dict_type(state, item).await.map(|rows_affected| ok_result_data(rows_affected))?
}

/*
 *更新字典类型状态
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
#[log]
pub async fn update_sys_dict_type_status(State(state): State<Arc<AppState>>, Valid(Json(item)): Valid<Json<UpdateDictTypeStatusReq>>) -> impl IntoResponse {
    DictTypeService::update_sys_dict_type_status(state, item).await.map(|rows_affected| ok_result_data(rows_affected))?
}

/*
 *查询字典类型详情
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
#[log]
pub async fn query_sys_dict_type_detail(State(state): State<Arc<AppState>>, Valid(Json(item)): Valid<Json<QueryDictTypeDetailReq>>) -> impl IntoResponse {
    DictTypeService::query_sys_dict_type_detail(state, item).await.map(|record| ok_result_data(record))?
}

/*
 *查询字典类型列
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
#[log]
pub async fn query_sys_dict_type_list(State(state): State<Arc<AppState>>, Valid(Json(item)): Valid<Json<QueryDictTypeListReq>>) -> impl IntoResponse {
    DictTypeService::query_sys_dict_type_list(state, item).await.map(|(list, total)| ok_result_page(list, total))?
}
