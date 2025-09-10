use crate::common::error::AppError;
use crate::common::result::{ok_result, ok_result_data, ok_result_page};
use crate::model::system::sys_dict_data_model::DictData;
use crate::vo::system::sys_dict_data_vo::*;
use crate::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use log::info;
use rbatis::plugin::page::PageRequest;
use rbs::value;
use std::sync::Arc;
/*
 *添加字典数据
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn add_sys_dict_data(State(state): State<Arc<AppState>>, Json(mut item): Json<DictDataReq>) -> impl IntoResponse {
    info!("add sys_dict_data params: {:?}", &item);
    let rb = &state.batis;

    if DictData::select_by_dict_label(rb, &item.dict_type, &item.dict_label).await?.is_some() {
        return Err(AppError::BusinessError("字典标签已存在"));
    }

    if DictData::select_by_dict_value(rb, &item.dict_type, &item.dict_value).await?.is_some() {
        return Err(AppError::BusinessError("字典键值已存在"));
    }

    item.id = None;
    DictData::insert(rb, &DictData::from(item)).await.map(|_| ok_result())?
}

/*
 *删除字典数据
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn delete_sys_dict_data(State(state): State<Arc<AppState>>, Json(item): Json<DeleteDictDataReq>) -> impl IntoResponse {
    info!("delete sys_dict_data params: {:?}", &item);
    let rb = &state.batis;

    DictData::delete_by_map(rb, value! {"id": &item.ids}).await.map(|_| ok_result())?
}

/*
 *更新字典数据
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn update_sys_dict_data(State(state): State<Arc<AppState>>, Json(item): Json<DictDataReq>) -> impl IntoResponse {
    info!("update sys_dict_data params: {:?}", &item);
    let rb = &state.batis;

    let id = item.id;

    if item.id.is_none() {
        return Err(AppError::BusinessError("主键不能为空"));
    }

    if DictData::select_by_id(rb, &id.unwrap_or_default()).await?.is_none() {
        return Err(AppError::BusinessError("字典数据不存在"));
    }

    if let Some(x) = DictData::select_by_dict_label(rb, &item.dict_type, &item.dict_label).await? {
        if x.id != item.id {
            return Err(AppError::BusinessError("字典标签已存在"));
        }
    }

    if let Some(x) = DictData::select_by_dict_value(rb, &item.dict_type, &item.dict_value).await? {
        if x.id != item.id {
            return Err(AppError::BusinessError("字典键值已存在"));
        }
    }

    DictData::update_by_map(rb, &DictData::from(item), value! {"id": &id}).await.map(|_| ok_result())?
}

/*
 *更新字典数据状态
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn update_sys_dict_data_status(State(state): State<Arc<AppState>>, Json(item): Json<UpdateDictDataStatusReq>) -> impl IntoResponse {
    info!("update sys_dict_data_status params: {:?}", &item);
    let rb = &state.batis;

    let update_sql = format!("update sys_dict_data set status = ? where id in ({})", item.ids.iter().map(|_| "?").collect::<Vec<&str>>().join(", "));

    let mut param = vec![value!(item.status)];
    param.extend(item.ids.iter().map(|&id| value!(id)));
    rb.exec(&update_sql, param).await.map(|_| ok_result())?
}

/*
 *查询字典数据详情
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn query_sys_dict_data_detail(State(state): State<Arc<AppState>>, Json(item): Json<QueryDictDataDetailReq>) -> impl IntoResponse {
    info!("query sys_dict_data_detail params: {:?}", &item);
    let rb = &state.batis;

    DictData::select_by_id(rb, &item.id).await?.map_or_else(
        || Err(AppError::BusinessError("字典数据不存在")),
        |x| {
            let data: DictDataResp = x.into();
            ok_result_data(data)
        },
    )
}

/*
 *查询字典数据列
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn query_sys_dict_data_list(State(state): State<Arc<AppState>>, Json(item): Json<QueryDictDataListReq>) -> impl IntoResponse {
    info!("query sys_dict_data_list params: {:?}", &item);
    let rb = &state.batis;

    let page = &PageRequest::new(item.page_no, item.page_size);

    DictData::select_dict_data_list(rb, page, &item)
        .await
        .map(|x| ok_result_page(x.records.into_iter().map(|x| x.into()).collect::<Vec<DictDataResp>>(), x.total))?
}
