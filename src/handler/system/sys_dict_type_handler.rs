use crate::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use rbatis::plugin::page::PageRequest;
use rbs::to_value;
use std::sync::Arc;

use crate::common::result::BaseResponse;
use crate::model::system::sys_dict_data_model::{count_dict_data_by_type, update_dict_data_type};
use crate::model::system::sys_dict_type_model::DictType;
use crate::utils::time_util::time_to_string;
use crate::vo::system::sys_dict_type_vo::*;
/*
 *添加字典类型
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn add_sys_dict_type(
    State(state): State<Arc<AppState>>,
    Json(item): Json<AddDictTypeReq>,
) -> impl IntoResponse {
    log::info!("add sys_dict_type params: {:?}", &item);
    let rb = &state.batis;

    let res_by_type = DictType::select_by_dict_type(rb, &item.dict_type).await;
    match res_by_type {
        Ok(r) => {
            if r.is_some() {
                return BaseResponse::<String>::err_result_msg(
                    "新增字典失败,字典类型已存在".to_string(),
                );
            }
        }
        Err(err) => return BaseResponse::<String>::err_result_msg(err.to_string()),
    }

    let sys_dict_type = DictType {
        dict_id: None,                           //字典主键
        dict_name: item.dict_name,               //字典名称
        dict_type: item.dict_type,               //字典类型
        status: item.status,                     //状态（0：停用，1:正常）
        remark: item.remark.unwrap_or_default(), //备注
        create_time: None,                       //创建时间
        update_time: None,                       //修改时间
    };

    let result = DictType::insert(rb, &sys_dict_type).await;

    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

/*
 *删除字典类型
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn delete_sys_dict_type(
    State(state): State<Arc<AppState>>,
    Json(item): Json<DeleteDictTypeReq>,
) -> impl IntoResponse {
    log::info!("delete sys_dict_type params: {:?}", &item);
    let rb = &state.batis;

    let ids = item.ids.clone();
    for id in ids {
        let dict_by_id = DictType::select_by_id(rb, &id).await;
        let p = match dict_by_id {
            Ok(p) => {
                if p.is_none() {
                    return BaseResponse::<String>::err_result_msg(
                        "字典类型不存在,不能删除".to_string(),
                    );
                } else {
                    p.unwrap()
                }
            }
            Err(err) => return BaseResponse::<String>::err_result_msg(err.to_string()),
        };

        let res = count_dict_data_by_type(rb, &p.dict_type)
            .await
            .unwrap_or_default();
        if res > 0 {
            let msg = format!("{}已分配,不能删除", p.dict_name);
            return BaseResponse::<String>::err_result_msg(msg);
        }
    }

    let result = DictType::delete_in_column(rb, "id", &item.ids).await;

    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

/*
 *更新字典类型
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn update_sys_dict_type(
    State(state): State<Arc<AppState>>,
    Json(item): Json<UpdateDictTypeReq>,
) -> impl IntoResponse {
    log::info!("update sys_dict_type params: {:?}", &item);
    let rb = &state.batis;

    let dict_by_id = DictType::select_by_id(rb, &item.dict_id).await;
    match dict_by_id {
        Ok(p) => {
            if p.is_none() {
                return BaseResponse::<String>::err_result_msg(
                    "更新字典失败,字典类型不存在".to_string(),
                );
            }
        }
        Err(err) => return BaseResponse::<String>::err_result_msg(err.to_string()),
    };

    let res_by_type = DictType::select_by_dict_type(rb, &item.dict_type).await;
    match res_by_type {
        Ok(r) => {
            if r.is_some() && r.clone().unwrap().dict_id.unwrap_or_default() != item.dict_id {
                return BaseResponse::<String>::err_result_msg(
                    "更新字典失败,字典类型已存在".to_string(),
                );
            }

            let dict_type = r.unwrap().dict_type;
            let _ = update_dict_data_type(rb, &*item.dict_type, &dict_type).await;
        }
        Err(err) => return BaseResponse::<String>::err_result_msg(err.to_string()),
    }

    let sys_dict_type = DictType {
        dict_id: Some(item.dict_id),             //字典主键
        dict_name: item.dict_name,               //字典名称
        dict_type: item.dict_type,               //字典类型
        status: item.status,                     //状态（0：停用，1:正常）
        remark: item.remark.unwrap_or_default(), //备注
        create_time: None,                       //创建时间
        update_time: None,                       //修改时间
    };

    let result = DictType::update_by_column(rb, &sys_dict_type, "dict_id").await;

    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

/*
 *更新字典类型状态
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn update_sys_dict_type_status(
    State(state): State<Arc<AppState>>,
    Json(item): Json<UpdateDictTypeStatusReq>,
) -> impl IntoResponse {
    log::info!("update sys_dict_type_status params: {:?}", &item);
    let rb = &state.batis;

    let update_sql = format!(
        "update sys_dict_type set status = ? where dict_id in ({})",
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
 *查询字典类型详情
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn query_sys_dict_type_detail(
    State(state): State<Arc<AppState>>,
    Json(item): Json<QueryDictTypeDetailReq>,
) -> impl IntoResponse {
    log::info!("query sys_dict_type_detail params: {:?}", &item);
    let rb = &state.batis;

    let result = DictType::select_by_id(rb, &item.id).await;

    match result {
        Ok(d) => {
            if d.is_none() {
                return BaseResponse::<QueryDictTypeDetailResp>::err_result_data(
                    QueryDictTypeDetailResp::new(),
                    "字典类型不存在".to_string(),
                );
            }
            let x = d.unwrap();

            let sys_dict_type = QueryDictTypeDetailResp {
                dict_id: x.dict_id.unwrap_or_default(),     //字典主键
                dict_name: x.dict_name,                     //字典名称
                dict_type: x.dict_type,                     //字典类型
                status: x.status,                           //状态（0：停用，1:正常）
                remark: x.remark,                           //备注
                create_time: time_to_string(x.create_time), //创建时间
                update_time: time_to_string(x.update_time), //修改时间
            };

            BaseResponse::<QueryDictTypeDetailResp>::ok_result_data(sys_dict_type)
        }
        Err(err) => BaseResponse::<QueryDictTypeDetailResp>::err_result_data(
            QueryDictTypeDetailResp::new(),
            err.to_string(),
        ),
    }
}

/*
 *查询字典类型列
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn query_sys_dict_type_list(
    State(state): State<Arc<AppState>>,
    Json(item): Json<QueryDictTypeListReq>,
) -> impl IntoResponse {
    log::info!("query sys_dict_type_list params: {:?}", &item);
    let rb = &state.batis;
    let dict_name = item.dict_name.as_deref().unwrap_or_default(); //字典名称
    let dict_type = item.dict_type.as_deref().unwrap_or_default(); //字典类型
    let status = item.status.unwrap_or(2); //状态（0：停用，1:正常）

    let page = &PageRequest::new(item.page_no, item.page_size);
    let result = DictType::select_dict_type_list(rb, page, dict_name, dict_type, status).await;

    let mut sys_dict_type_list_data: Vec<DictTypeListDataResp> = Vec::new();
    match result {
        Ok(d) => {
            let total = d.total;

            for x in d.records {
                sys_dict_type_list_data.push(DictTypeListDataResp {
                    dict_id: x.dict_id.unwrap_or_default(),     //字典主键
                    dict_name: x.dict_name,                     //字典名称
                    dict_type: x.dict_type,                     //字典类型
                    status: x.status,                           //状态（0：停用，1:正常）
                    remark: x.remark,                           //备注
                    create_time: time_to_string(x.create_time), //创建时间
                    update_time: time_to_string(x.update_time), //修改时间
                })
            }

            BaseResponse::ok_result_page(sys_dict_type_list_data, total)
        }
        Err(err) => BaseResponse::err_result_page(DictTypeListDataResp::new(), err.to_string()),
    }
}
