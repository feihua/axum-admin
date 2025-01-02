use crate::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use rbatis::plugin::page::PageRequest;
use rbs::to_value;
use std::sync::Arc;

use crate::common::result::BaseResponse;
use crate::model::system::sys_dict_data_model::DictData;
use crate::vo::system::sys_dict_data_vo::*;

/*
 *添加字典数据
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn add_sys_dict_data(
    State(state): State<Arc<AppState>>,
    Json(item): Json<AddDictDataReq>,
) -> impl IntoResponse {
    log::info!("add sys_dict_data params: {:?}", &item);
    let rb = &state.batis;

    let sys_dict_data = DictData {
        dict_code: None,                         //字典编码
        dict_sort: item.dict_sort,               //字典排序
        dict_label: item.dict_label,             //字典标签
        dict_value: item.dict_value,             //字典键值
        dict_type: item.dict_type,               //字典类型
        css_class: item.css_class,               //样式属性（其他样式扩展）
        list_class: item.list_class,             //格回显样式
        is_default: item.is_default,             //是否默认（Y是 N否）
        status: item.status,                     //门状态（0：停用，1:正常）
        remark: item.remark.unwrap_or_default(), //备注
        create_time: None,                       //创建时间
        update_time: None,                       //修改时间
    };

    let result = DictData::insert(rb, &sys_dict_data).await;

    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

/*
 *删除字典数据
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn delete_sys_dict_data(
    State(state): State<Arc<AppState>>,
    Json(item): Json<DeleteDictDataReq>,
) -> impl IntoResponse {
    log::info!("delete sys_dict_data params: {:?}", &item);
    let rb = &state.batis;

    let result = DictData::delete_in_column(rb, "dict_code", &item.ids).await;

    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

/*
 *更新字典数据
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn update_sys_dict_data(
    State(state): State<Arc<AppState>>,
    Json(item): Json<UpdateDictDataReq>,
) -> impl IntoResponse {
    log::info!("update sys_dict_data params: {:?}", &item);
    let rb = &state.batis;

    let sys_dict_data = DictData {
        dict_code: Some(item.dict_code),         //字典编码
        dict_sort: item.dict_sort,               //字典排序
        dict_label: item.dict_label,             //字典标签
        dict_value: item.dict_value,             //字典键值
        dict_type: item.dict_type,               //字典类型
        css_class: item.css_class,               //样式属性（其他样式扩展）
        list_class: item.list_class,             //格回显样式
        is_default: item.is_default,             //是否默认（Y是 N否）
        status: item.status,                     //门状态（0：停用，1:正常）
        remark: item.remark.unwrap_or_default(), //备注
        create_time: None,                       //创建时间
        update_time: None,                       //修改时间
    };

    let result = DictData::update_by_column(rb, &sys_dict_data, "dict_code").await;

    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

/*
 *更新字典数据状态
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn update_sys_dict_data_status(
    State(state): State<Arc<AppState>>,
    Json(item): Json<UpdateDictDataStatusReq>,
) -> impl IntoResponse {
    log::info!("update sys_dict_data_status params: {:?}", &item);
    let rb = &state.batis;

    let update_sql = format!(
        "update sys_dict_data set status = ? where dict_code in ({})",
        item.ids
            .iter()
            .map(|_| "?")
            .collect::<Vec<&str>>()
            .join(", ")
    );

    let mut param = vec![to_value!(item.status)];
    param.extend(item.ids.iter().map(|&id| to_value!(id)));
    let result = rb.exec(&update_sql, param).await;

    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

/*
 *查询字典数据详情
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn query_sys_dict_data_detail(
    State(state): State<Arc<AppState>>,
    Json(item): Json<QueryDictDataDetailReq>,
) -> impl IntoResponse {
    log::info!("query sys_dict_data_detail params: {:?}", &item);
    let rb = &state.batis;

    let result = DictData::select_by_id(rb, &item.id).await;

    match result {
        Ok(d) => {
            if d.is_none() {
                return BaseResponse::<QueryDictDataDetailResp>::err_result_data(
                    QueryDictDataDetailResp::new(),
                    "字典数据不存在".to_string(),
                );
            }

            let x = d.unwrap();

            let sys_dict_data = QueryDictDataDetailResp {
                dict_code: x.dict_code.unwrap_or_default(), //字典编码
                dict_sort: x.dict_sort,                     //字典排序
                dict_label: x.dict_label,                   //字典标签
                dict_value: x.dict_value,                   //字典键值
                dict_type: x.dict_type,                     //字典类型
                css_class: x.css_class,                     //样式属性（其他样式扩展）
                list_class: x.list_class,                   //格回显样式
                is_default: x.is_default,                   //是否默认（Y是 N否）
                status: x.status,                           //门状态（0：停用，1:正常）
                remark: x.remark,                           //备注
                create_time: x.create_time.unwrap().0.to_string(), //创建时间
                update_time: x.update_time.unwrap().0.to_string(), //修改时间
            };

            BaseResponse::<QueryDictDataDetailResp>::ok_result_data(sys_dict_data)
        }
        Err(err) => BaseResponse::<QueryDictDataDetailResp>::err_result_data(
            QueryDictDataDetailResp::new(),
            err.to_string(),
        ),
    }
}

/*
 *查询字典数据列
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn query_sys_dict_data_list(
    State(state): State<Arc<AppState>>,
    Json(item): Json<QueryDictDataListReq>,
) -> impl IntoResponse {
    log::info!("query sys_dict_data_list params: {:?}", &item);
    let rb = &state.batis;
    let dict_label = item.dict_label.as_deref().unwrap_or_default(); //字典标签
    let dict_type = item.dict_type.as_deref().unwrap_or_default(); //字典类型
    let status = item.status.unwrap_or(2); //门状态（0：停用，1:正常）

    let page = &PageRequest::new(item.page_no, item.page_size);
    let result = DictData::select_dict_data_list(rb, page, dict_label, dict_type, status).await;

    let mut sys_dict_data_list_data: Vec<DictDataListDataResp> = Vec::new();
    match result {
        Ok(d) => {
            let total = d.total;

            for x in d.records {
                sys_dict_data_list_data.push(DictDataListDataResp {
                    dict_code: x.dict_code.unwrap_or_default(), //字典编码
                    dict_sort: x.dict_sort,                     //字典排序
                    dict_label: x.dict_label,                   //字典标签
                    dict_value: x.dict_value,                   //字典键值
                    dict_type: x.dict_type,                     //字典类型
                    css_class: x.css_class,                     //样式属性（其他样式扩展）
                    list_class: x.list_class,                   //格回显样式
                    is_default: x.is_default,                   //是否默认（Y是 N否）
                    status: x.status,                           //门状态（0：停用，1:正常）
                    remark: x.remark,                           //备注
                    create_time: x.create_time.unwrap().0.to_string(), //创建时间
                    update_time: x.update_time.unwrap().0.to_string(), //修改时间
                })
            }

            BaseResponse::ok_result_page(sys_dict_data_list_data, total)
        }
        Err(err) => BaseResponse::err_result_page(DictDataListDataResp::new(), err.to_string()),
    }
}
