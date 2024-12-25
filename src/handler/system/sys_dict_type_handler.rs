use crate::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use rbatis::plugin::page::PageRequest;
use rbs::to_value;
use std::sync::Arc;

use crate::common::result::BaseResponse;
use crate::model::system::sys_dict_type_model::DictType;
use crate::vo::system::sys_dict_type_vo::*;

/*
 *添加字典类型表
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn add_sys_dict_type(
    State(state): State<Arc<AppState>>,
    Json(item): Json<AddDictTypeReq>,
) -> impl IntoResponse {
    log::info!("add sys_dict_type params: {:?}", &item);
    let rb = &state.batis;

    let sys_dict_type = DictType {
        dict_id: None,                           //字典主键
        dict_name: item.dict_name,               //字典名称
        dict_type: item.dict_type,               //字典类型
        status: item.status,                     //门状态（0：停用，1:正常）
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
 *删除字典类型表
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn delete_sys_dict_type(
    State(state): State<Arc<AppState>>,
    Json(item): Json<DeleteDictTypeReq>,
) -> impl IntoResponse {
    log::info!("delete sys_dict_type params: {:?}", &item);
    let rb = &state.batis;

    let result = DictType::delete_in_column(rb, "id", &item.ids).await;

    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

/*
 *更新字典类型表
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn update_sys_dict_type(
    State(state): State<Arc<AppState>>,
    Json(item): Json<UpdateDictTypeReq>,
) -> impl IntoResponse {
    log::info!("update sys_dict_type params: {:?}", &item);
    let rb = &state.batis;

    let sys_dict_type = DictType {
        dict_id: Some(item.dict_id),             //字典主键
        dict_name: item.dict_name,               //字典名称
        dict_type: item.dict_type,               //字典类型
        status: item.status,                     //门状态（0：停用，1:正常）
        remark: item.remark.unwrap_or_default(), //备注
        create_time: None,                       //创建时间
        update_time: None,                       //修改时间
    };

    let result = DictType::update_by_column(rb, &sys_dict_type, "id").await;

    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

/*
 *更新字典类型表状态
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
        "update sys_dict_type set status = ? where id in ({})",
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
 *查询字典类型表详情
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
            let x = d.unwrap();

            let sys_dict_type = QueryDictTypeDetailResp {
                dict_id: x.dict_id.unwrap(),                       //字典主键
                dict_name: x.dict_name,                            //字典名称
                dict_type: x.dict_type,                            //字典类型
                status: x.status,                                  //门状态（0：停用，1:正常）
                remark: x.remark,                                  //备注
                create_time: x.create_time.unwrap().0.to_string(), //创建时间
                update_time: x.update_time.unwrap().0.to_string(), //修改时间
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
 *查询字典类型表列表
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn query_sys_dict_type_list(
    State(state): State<Arc<AppState>>,
    Json(item): Json<QueryDictTypeListReq>,
) -> impl IntoResponse {
    log::info!("query sys_dict_type_list params: {:?}", &item);
    let rb = &state.batis;
    //let dict_name = item.dict_name.as_deref().unwrap_or_default(); //字典名称
    //let dict_type = item.dict_type.as_deref().unwrap_or_default(); //字典类型
    //let status = item.status.unwrap_or(2); //门状态（0：停用，1:正常）

    let page = &PageRequest::new(item.page_no, item.page_size);
    let result = DictType::select_page(rb, page).await;

    let mut sys_dict_type_list_data: Vec<DictTypeListDataResp> = Vec::new();
    match result {
        Ok(d) => {
            let total = d.total;

            for x in d.records {
                sys_dict_type_list_data.push(DictTypeListDataResp {
                    dict_id: x.dict_id.unwrap(),                       //字典主键
                    dict_name: x.dict_name,                            //字典名称
                    dict_type: x.dict_type,                            //字典类型
                    status: x.status,                                  //门状态（0：停用，1:正常）
                    remark: x.remark,                                  //备注
                    create_time: x.create_time.unwrap().0.to_string(), //创建时间
                    update_time: x.update_time.unwrap().0.to_string(), //修改时间
                })
            }

            BaseResponse::ok_result_page(sys_dict_type_list_data, total)
        }
        Err(err) => BaseResponse::err_result_page(DictTypeListDataResp::new(), err.to_string()),
    }
}
