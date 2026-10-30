use crate::common::error::AppError;
use crate::common::error::AppError::BusinessError;
use crate::model::system::sys_dict_data_model::DictData;
use crate::vo::system::sys_dict_data_vo::*;
use crate::AppState;
use curd_trait::BaseCurd;
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
    pub async fn add_sys_dict_data(state: Arc<AppState>, mut item: DictDataReq) -> Result<u64, AppError> {
        let rb = &state.batis;

        let condition = value! {"dict_type":&item.dict_type,"dict_label":&item.dict_label};
        if DictData::select_by_map(rb, condition).await?.len() > 0 {
            return Err(BusinessError("字典标签已存在"));
        }

        let condition1 = value! {"dict_type":&item.dict_type,"dict_value":&item.dict_value};
        if DictData::select_by_map(rb, condition1).await?.len() > 0 {
            return Err(BusinessError("字典键值已存在"));
        }

        item.id = None;
        DictData::add(rb, &DictData::from(item)).await.map(|id| Ok(id.unwrap_or_default()))?
    }

    /*
     *删除字典数据
     *author：刘飞华
     *date：2024/12/25 11:36:48
     */
    pub async fn delete_sys_dict_data(state: Arc<AppState>, item: DeleteDictDataReq) -> Result<u64, AppError> {
        let rb = &state.batis;
        DictData::delete(rb, item.ids).await.map(|x| Ok(x))?
    }

    /*
     *更新字典数据
     *author：刘飞华
     *date：2024/12/25 11:36:48
     */
    pub async fn update_sys_dict_data(state: Arc<AppState>, item: DictDataReq) -> Result<u64, AppError> {
        let rb = &state.batis;
        let id = item.id;

        if id.is_none() {
            return Err(BusinessError("主键不能为空"));
        }

        if DictData::select_by_id(rb, &id.unwrap_or_default()).await?.is_none() {
            return Err(BusinessError("字典数据不存在"));
        }

        let condition = value! {"dict_type":&item.dict_type,"dict_label":&item.dict_label,"id !=":id};
        if DictData::select_by_map(rb, condition).await?.len() > 0 {
            return Err(BusinessError("字典标签已存在"));
        }

        let condition = value! {"dict_type":&item.dict_type,"dict_value":&item.dict_value,"id !=":id};
        if DictData::select_by_map(rb, condition).await?.len() > 0 {
            return Err(BusinessError("字典键值已存在"));
        }

        DictData::update(rb, &DictData::from(item), id.unwrap_or_default()).await.map(|x| Ok(x))?
    }

    /*
     *更新字典数据状态
     *author：刘飞华
     *date：2024/12/25 11:36:48
     */
    pub async fn update_sys_dict_data_status(state: Arc<AppState>, item: UpdateDictDataStatusReq) -> Result<u64, AppError> {
        let rb = &state.batis;
        DictData::update_status(rb, item.ids, item.status).await.map(|x| Ok(x))?
    }

    /*
     *查询字典数据详情
     *author：刘飞华
     *date：2024/12/25 11:36:48
     */
    pub async fn query_sys_dict_data_detail(state: Arc<AppState>, item: QueryDictDataDetailReq) -> Result<DictDataResp, AppError> {
        let rb = &state.batis;
        let result = DictData::select_by_id(rb, &item.id).await?;

        match result {
            None => Err(BusinessError("字典数据不存在")),
            Some(x) => Ok(x.into()),
        }
    }

    /*
     *查询字典数据列
     *author：刘飞华
     *date：2024/12/25 11:36:48
     */
    pub async fn query_sys_dict_data_list(state: Arc<AppState>, item: QueryDictDataListReq) -> Result<(Vec<DictDataResp>, u64), AppError> {
        let rb = &state.batis;
        let page = &PageRequest::new(item.page_no, item.page_size);

        DictData::select_by_page(rb, page, &item)
            .await
            .map(|x| Ok((x.records.into_iter().map(|x| x.into()).collect::<Vec<DictDataResp>>(), x.total)))?
    }
}
