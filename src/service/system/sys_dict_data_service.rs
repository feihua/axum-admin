use crate::common::error::AppError;
use crate::common::result::{ok_result, ok_result_data, ok_result_page};
use crate::model::system::sys_dict_data_model::DictData;
use crate::vo::system::sys_dict_data_vo::*;
use crate::AppState;
use axum::response::IntoResponse;
use rbatis::plugin::page::PageRequest;
use rbs::value;
use std::sync::Arc;

pub struct DictDataService;

impl DictDataService {
    /*
     *添加字典数据
     *author：刘飞华
     *date：2024/12/25 11:36:48
     */
    pub async fn add_sys_dict_data(state: Arc<AppState>, mut item: DictDataReq) -> impl IntoResponse {
        let rb = &state.batis;
        let dict_type = &item.dict_type;
        let condition = value! {"dict_type":dict_type,"dict_label":&item.dict_label};
        if DictData::select_by_map(rb, condition).await?.len() > 0 {
            return Err(AppError::BusinessError("字典标签已存在"));
        }

        let condition1 = value! {"dict_type":dict_type,"dict_value":&item.dict_value};
        if DictData::select_by_map(rb, condition1).await?.len() > 0 {
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
    pub async fn delete_sys_dict_data(state: Arc<AppState>, item: DeleteDictDataReq) -> impl IntoResponse {
        let rb = &state.batis;
        DictData::delete_by_map(rb, value! {"id": &item.ids}).await.map(|_| ok_result())?
    }

    /*
     *更新字典数据
     *author：刘飞华
     *date：2024/12/25 11:36:48
     */
    pub async fn update_sys_dict_data(state: Arc<AppState>, item: DictDataReq) -> impl IntoResponse {
        let rb = &state.batis;
        let id = item.id;

        if id.is_none() {
            return Err(AppError::BusinessError("主键不能为空"));
        }

        if DictData::select_by_id(rb, &id.unwrap_or_default()).await?.is_none() {
            return Err(AppError::BusinessError("字典数据不存在"));
        }

        let dict_type = &item.dict_type;
        let condition = value! {"dict_type":dict_type,"dict_label":&item.dict_label,"id !=":id};
        if DictData::select_by_map(rb, condition).await?.len() > 0 {
            return Err(AppError::BusinessError("字典标签已存在"));
        }

        let condition1 = value! {"dict_type":dict_type,"dict_value":&item.dict_value,"id !=":id};
        if DictData::select_by_map(rb, condition1).await?.len() > 0 {
            return Err(AppError::BusinessError("字典键值已存在"));
        }

        DictData::update_by_map(rb, &DictData::from(item), value! {"id": &id}).await.map(|_| ok_result())?
    }

    /*
     *更新字典数据状态
     *author：刘飞华
     *date：2024/12/25 11:36:48
     */
    pub async fn update_sys_dict_data_status(state: Arc<AppState>, item: UpdateDictDataStatusReq) -> impl IntoResponse {
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
    pub async fn query_sys_dict_data_detail(state: Arc<AppState>, item: QueryDictDataDetailReq) -> impl IntoResponse {
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
    pub async fn query_sys_dict_data_list(state: Arc<AppState>, item: QueryDictDataListReq) -> impl IntoResponse {
        let rb = &state.batis;
        let page = &PageRequest::new(item.page_no, item.page_size);

        DictData::select_by_page(rb, page, &item)
            .await
            .map(|x| ok_result_page(x.records.into_iter().map(|x| x.into()).collect::<Vec<DictDataResp>>(), x.total))?
    }
}
