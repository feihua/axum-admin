use crate::common::error::AppError;
use crate::common::error::AppError::BusinessError;
use crate::model::system::sys_dict_data_model::DictData;
use crate::model::system::sys_dict_type_model::DictType;
use crate::vo::system::sys_dict_type_vo::*;
use crate::AppState;
use curd_trait::BaseCurd;
use rbatis::plugin::page::PageRequest;
use rbs::value;
use std::sync::Arc;

pub struct DictTypeService;

impl DictTypeService {
    /*
     *添加字典类型
     *author：刘飞华
     *date：2024/12/25 11:36:48
     */
    pub async fn add_sys_dict_type(state: Arc<AppState>, mut item: DictTypeReq) -> Result<u64, AppError> {
        let rb = &state.batis;
        if DictType::select_by_map(rb, value! {"dict_type": &item.dict_type}).await?.len() > 0 {
            return Err(BusinessError("字典类型已存在"));
        }

        item.id = None;
        DictType::add(rb, &DictType::from(item)).await.map(|id| Ok(id.unwrap_or_default()))?
    }

    /*
     *删除字典类型
     *author：刘飞华
     *date：2024/12/25 11:36:48
     */
    pub async fn delete_sys_dict_type(state: Arc<AppState>, item: DeleteDictTypeReq) -> Result<u64, AppError> {
        let rb = &state.batis;
        let ids = item.ids;
        for id in &ids {
            let dict_type = match DictType::select_by_id(rb, id).await? {
                None => return Err(BusinessError("字典类型不存在,不能删除")),
                Some(p) => p.dict_type,
            };

            if DictData::count_dict_data_by_type(rb, &dict_type).await? > 0 {
                return Err(BusinessError("已分配,不能删除"));
            }
        }

        DictType::delete(rb, ids).await.map(|x| Ok(x))?
    }

    /*
     *更新字典类型
     *author：刘飞华
     *date：2024/12/25 11:36:48
     */
    pub async fn update_sys_dict_type(state: Arc<AppState>, item: DictTypeReq) -> Result<u64, AppError> {
        let rb = &state.batis;
        let id = item.id;

        if id.is_none() {
            return Err(BusinessError("主键不能为空"));
        }

        let option = DictType::select_by_id(rb, &id.unwrap_or_default()).await?;
        if option.is_none() {
            return Err(BusinessError("字典类型不存在"));
        }

        if DictType::select_by_map(rb, value! {"dict_type": &item.dict_type,"id !=": id}).await?.len() > 0 {
            return Err(BusinessError("字典类型已存在"));
        }

        let dict_type = option.unwrap().dict_type;
        if dict_type != item.dict_type {
            DictData::update_dict_data_type(rb, &*item.dict_type, &dict_type).await?;
        }

        DictType::update(rb, &DictType::from(item), id.unwrap_or_default()).await.map(|x| Ok(x))?
    }

    /*
     *更新字典类型状态
     *author：刘飞华
     *date：2024/12/25 11:36:48
     */
    pub async fn update_sys_dict_type_status(state: Arc<AppState>, item: UpdateDictTypeStatusReq) -> Result<u64, AppError> {
        let rb = &state.batis;
        DictType::update_status(rb, "sys_dict_type", item.ids, item.status).await.map(|x| Ok(x))?
    }

    /*
     *查询字典类型详情
     *author：刘飞华
     *date：2024/12/25 11:36:48
     */
    pub async fn query_sys_dict_type_detail(state: Arc<AppState>, item: QueryDictTypeDetailReq) -> Result<DictTypeResp, AppError> {
        let rb = &state.batis;
        let result = DictType::select_by_id(rb, &item.id).await?;

        match result {
            None => Err(BusinessError("字典类型表不存在")),
            Some(x) => Ok(x.into()),
        }
    }

    /*
     *查询字典类型列
     *author：刘飞华
     *date：2024/12/25 11:36:48
     */
    pub async fn query_sys_dict_type_list(state: Arc<AppState>, item: QueryDictTypeListReq) -> Result<(Vec<DictTypeResp>, u64), AppError> {
        let rb = &state.batis;
        let page = &PageRequest::new(item.page_no, item.page_size);

        DictType::select_by_page(rb, page, &item)
            .await
            .map(|x| Ok((x.records.into_iter().map(|x| x.into()).collect::<Vec<DictTypeResp>>(), x.total)))?
    }
}
