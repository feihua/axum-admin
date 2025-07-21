use crate::common::error::AppError;
use crate::common::result::BaseResponse;
use crate::model::system::sys_dict_data_model::DictData;
use crate::utils::time_util::time_to_string;
use crate::vo::system::sys_dict_data_vo::*;
use crate::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use rbatis::plugin::page::PageRequest;
use rbs::value;
use std::sync::Arc;

/*
 *添加字典数据
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn add_sys_dict_data(State(state): State<Arc<AppState>>, Json(item): Json<AddDictDataReq>) -> impl IntoResponse {
    log::info!("add sys_dict_data params: {:?}", &item);
    let rb = &state.batis;

    if DictData::select_by_dict_label(rb, &item.dict_type, &item.dict_label).await?.is_some() {
        return Err(AppError::BusinessError("字典标签已存在".to_string()));
    }

    if DictData::select_by_dict_value(rb, &item.dict_type, &item.dict_value).await?.is_some() {
        return Err(AppError::BusinessError("字典键值已存在".to_string()));
    }

    let sys_dict_data = DictData {
        dict_code: None,                         //字典编码
        dict_sort: item.dict_sort,               //字典排序
        dict_label: item.dict_label,             //字典标签
        dict_value: item.dict_value,             //字典键值
        dict_type: item.dict_type,               //字典类型
        css_class: item.css_class,               //样式属性（其他样式扩展）
        list_class: item.list_class,             //格回显样式
        is_default: item.is_default,             //是否默认（Y是 N否）
        status: item.status,                     //状态（0：停用，1:正常）
        remark: item.remark.unwrap_or_default(), //备注
        create_time: None,                       //创建时间
        update_time: None,                       //修改时间
    };

    DictData::insert(rb, &sys_dict_data).await?;
    BaseResponse::<String>::ok_result()
}

/*
 *删除字典数据
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn delete_sys_dict_data(State(state): State<Arc<AppState>>, Json(item): Json<DeleteDictDataReq>) -> impl IntoResponse {
    log::info!("delete sys_dict_data params: {:?}", &item);
    let rb = &state.batis;

    DictData::delete_by_map(rb, value! {"dict_code": &item.ids}).await?;
    BaseResponse::<String>::ok_result()
}

/*
 *更新字典数据
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn update_sys_dict_data(State(state): State<Arc<AppState>>, Json(item): Json<UpdateDictDataReq>) -> impl IntoResponse {
    log::info!("update sys_dict_data params: {:?}", &item);
    let rb = &state.batis;

    if DictData::select_by_id(rb, &item.dict_code).await?.is_none() {
        return Err(AppError::BusinessError("字典数据不存在".to_string()));
    }

    if let Some(x) = DictData::select_by_dict_label(rb, &item.dict_type, &item.dict_label).await? {
        if x.dict_code.unwrap_or_default() != item.dict_code {
            return Err(AppError::BusinessError("字典标签已存在".to_string()));
        }
    }

    if let Some(x) = DictData::select_by_dict_value(rb, &item.dict_type, &item.dict_value).await? {
        if x.dict_code.unwrap_or_default() != item.dict_code {
            return Err(AppError::BusinessError("字典键值已存在".to_string()));
        }
    }

    let sys_dict_data = DictData {
        dict_code: Some(item.dict_code),         //字典编码
        dict_sort: item.dict_sort,               //字典排序
        dict_label: item.dict_label,             //字典标签
        dict_value: item.dict_value,             //字典键值
        dict_type: item.dict_type,               //字典类型
        css_class: item.css_class,               //样式属性（其他样式扩展）
        list_class: item.list_class,             //格回显样式
        is_default: item.is_default,             //是否默认（Y是 N否）
        status: item.status,                     //状态（0：停用，1:正常）
        remark: item.remark.unwrap_or_default(), //备注
        create_time: None,                       //创建时间
        update_time: None,                       //修改时间
    };

    DictData::update_by_map(rb, &sys_dict_data, value! {"dict_code": &item.dict_code}).await?;
    BaseResponse::<String>::ok_result()
}

/*
 *更新字典数据状态
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn update_sys_dict_data_status(State(state): State<Arc<AppState>>, Json(item): Json<UpdateDictDataStatusReq>) -> impl IntoResponse {
    log::info!("update sys_dict_data_status params: {:?}", &item);
    let rb = &state.batis;

    let update_sql = format!(
        "update sys_dict_data set status = ? where dict_code in ({})",
        item.ids.iter().map(|_| "?").collect::<Vec<&str>>().join(", ")
    );

    let mut param = vec![value!(item.status)];
    param.extend(item.ids.iter().map(|&id| value!(id)));
    rb.exec(&update_sql, param).await?;
    BaseResponse::<String>::ok_result()
}

/*
 *查询字典数据详情
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn query_sys_dict_data_detail(State(state): State<Arc<AppState>>, Json(item): Json<QueryDictDataDetailReq>) -> impl IntoResponse {
    log::info!("query sys_dict_data_detail params: {:?}", &item);
    let rb = &state.batis;

    match DictData::select_by_id(rb, &item.id).await? {
        None => Err(AppError::BusinessError("字典数据不存在".to_string())),
        Some(x) => {
            let sys_dict_data = QueryDictDataDetailResp {
                dict_code: x.dict_code.unwrap_or_default(), //字典编码
                dict_sort: x.dict_sort,                     //字典排序
                dict_label: x.dict_label,                   //字典标签
                dict_value: x.dict_value,                   //字典键值
                dict_type: x.dict_type,                     //字典类型
                css_class: x.css_class,                     //样式属性（其他样式扩展）
                list_class: x.list_class,                   //格回显样式
                is_default: x.is_default,                   //是否默认（Y是 N否）
                status: x.status,                           //状态（0：停用，1:正常）
                remark: x.remark,                           //备注
                create_time: time_to_string(x.create_time), //创建时间
                update_time: time_to_string(x.update_time), //修改时间
            };

            BaseResponse::<QueryDictDataDetailResp>::ok_result_data(sys_dict_data)
        }
    }
}

/*
 *查询字典数据列
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn query_sys_dict_data_list(State(state): State<Arc<AppState>>, Json(item): Json<QueryDictDataListReq>) -> impl IntoResponse {
    log::info!("query sys_dict_data_list params: {:?}", &item);
    let rb = &state.batis;

    let page = &PageRequest::new(item.page_no, item.page_size);
    let d = DictData::select_dict_data_list(rb, page, &item).await?;

    let mut list: Vec<DictDataListDataResp> = Vec::new();

    let total = d.total;

    for x in d.records {
        list.push(DictDataListDataResp {
            dict_code: x.dict_code.unwrap_or_default(), //字典编码
            dict_sort: x.dict_sort,                     //字典排序
            dict_label: x.dict_label,                   //字典标签
            dict_value: x.dict_value,                   //字典键值
            dict_type: x.dict_type,                     //字典类型
            css_class: x.css_class,                     //样式属性（其他样式扩展）
            list_class: x.list_class,                   //格回显样式
            is_default: x.is_default,                   //是否默认（Y是 N否）
            status: x.status,                           //状态（0：停用，1:正常）
            remark: x.remark,                           //备注
            create_time: time_to_string(x.create_time), //创建时间
            update_time: time_to_string(x.update_time), //修改时间
        })
    }

    BaseResponse::ok_result_page(list, total)
}
