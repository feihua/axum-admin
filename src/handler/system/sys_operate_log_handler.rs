use crate::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use rbatis::plugin::page::PageRequest;
use rbatis::rbdc::datetime::DateTime;
use rbs::to_value;
use std::sync::Arc;

use crate::common::result::BaseResponse;
use crate::model::system::sys_operate_log_model::OperateLog;
use crate::vo::system::sys_operate_log_vo::*;
use crate::vo::system::*;

/*
 *添加操作日志记录
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
pub async fn add_sys_operate_log(
    State(state): State<Arc<AppState>>,
    Json(item): Json<AddOperateLogReq>,
) -> impl IntoResponse {
    log::info!("add sys_operate_log params: {:?}", &item);
    let rb = &state.batis;

    let sys_operate_log = OperateLog {
        id: None,                                //日志主键
        title: item.title,                       //模块标题
        business_type: item.business_type,       //业务类型（0其它 1新增 2修改 3删除）
        method: item.method,                     //方法名称
        request_method: item.request_method,     //请求方式
        operator_type: item.operator_type,       //操作类别（0其它 1后台用户 2手机端用户）
        operate_name: item.operate_name,         //操作人员
        dept_name: item.dept_name,               //部门名称
        operate_url: item.operate_url,           //请求URL
        operate_ip: item.operate_ip,             //主机地址
        operate_location: item.operate_location, //操作地点
        operate_param: item.operate_param,       //请求参数
        json_result: item.json_result,           //返回参数
        status: item.status,                     //操作状态(0:异常,正常)
        error_msg: item.error_msg,               //错误消息
        operate_time: item.operate_time,         //操作时间
        cost_time: item.cost_time,               //消耗时间
    };

    let result = OperateLog::insert(rb, &sys_operate_log).await;

    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

/*
 *删除操作日志记录
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
pub async fn delete_sys_operate_log(
    State(state): State<Arc<AppState>>,
    Json(item): Json<DeleteOperateLogReq>,
) -> impl IntoResponse {
    log::info!("delete sys_operate_log params: {:?}", &item);
    let rb = &state.batis;

    let result = OperateLog::delete_in_column(rb, "id", &item.ids).await;

    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

/*
 *更新操作日志记录
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
pub async fn update_sys_operate_log(
    State(state): State<Arc<AppState>>,
    Json(item): Json<UpdateOperateLogReq>,
) -> impl IntoResponse {
    log::info!("update sys_operate_log params: {:?}", &item);
    let rb = &state.batis;

    let sys_operate_log = OperateLog {
        id: Some(item.id),                       //日志主键
        title: item.title,                       //模块标题
        business_type: item.business_type,       //业务类型（0其它 1新增 2修改 3删除）
        method: item.method,                     //方法名称
        request_method: item.request_method,     //请求方式
        operator_type: item.operator_type,       //操作类别（0其它 1后台用户 2手机端用户）
        operate_name: item.operate_name,         //操作人员
        dept_name: item.dept_name,               //部门名称
        operate_url: item.operate_url,           //请求URL
        operate_ip: item.operate_ip,             //主机地址
        operate_location: item.operate_location, //操作地点
        operate_param: item.operate_param,       //请求参数
        json_result: item.json_result,           //返回参数
        status: item.status,                     //操作状态(0:异常,正常)
        error_msg: item.error_msg,               //错误消息
        operate_time: item.operate_time,         //操作时间
        cost_time: item.cost_time,               //消耗时间
    };

    let result = OperateLog::update_by_column(rb, &sys_operate_log, "id").await;

    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

/*
 *更新操作日志记录状态
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
pub async fn update_sys_operate_log_status(
    State(state): State<Arc<AppState>>,
    Json(item): Json<UpdateOperateLogStatusReq>,
) -> impl IntoResponse {
    log::info!("update sys_operate_log_status params: {:?}", &item);
    let rb = &state.batis;

    let update_sql = format!(
        "update sys_operate_log set status = ? where id in ({})",
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
 *查询操作日志记录详情
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
pub async fn query_sys_operate_log_detail(
    State(state): State<Arc<AppState>>,
    Json(item): Json<QueryOperateLogDetailReq>,
) -> impl IntoResponse {
    log::info!("query sys_operate_log_detail params: {:?}", &item);
    let rb = &state.batis;

    let result = OperateLog::select_by_id(rb, &item.id).await;

    match result {
        Ok(d) => {
            let x = d.unwrap();

            let sys_operate_log = QueryOperateLogDetailResp {
                id: x.id.unwrap(),                                        //日志主键
                title: x.title.unwrap_or_default(),                       //模块标题
                business_type: x.business_type.unwrap_or_default(), //业务类型（0其它 1新增 2修改 3删除）
                method: x.method.unwrap_or_default(),               //方法名称
                request_method: x.request_method.unwrap_or_default(), //请求方式
                operator_type: x.operator_type.unwrap_or_default(), //操作类别（0其它 1后台用户 2手机端用户）
                operate_name: x.operate_name.unwrap_or_default(),   //操作人员
                dept_name: x.dept_name.unwrap_or_default(),         //部门名称
                operate_url: x.operate_url.unwrap_or_default(),     //请求URL
                operate_ip: x.operate_ip.unwrap_or_default(),       //主机地址
                operate_location: x.operate_location.unwrap_or_default(), //操作地点
                operate_param: x.operate_param.unwrap_or_default(), //请求参数
                json_result: x.json_result.unwrap_or_default(),     //返回参数
                status: x.status.unwrap_or_default(),               //操作状态(0:异常,正常)
                error_msg: x.error_msg.unwrap_or_default(),         //错误消息
                operate_time: x.operate_time.to_string(),           //操作时间
                cost_time: x.cost_time.unwrap_or_default(),         //消耗时间
            };

            BaseResponse::<QueryOperateLogDetailResp>::ok_result_data(sys_operate_log)
        }
        Err(err) => BaseResponse::<QueryOperateLogDetailResp>::err_result_data(
            QueryOperateLogDetailResp::new(),
            err.to_string(),
        ),
    }
}

/*
 *查询操作日志记录列表
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
pub async fn query_sys_operate_log_list(
    State(state): State<Arc<AppState>>,
    Json(item): Json<QueryOperateLogListReq>,
) -> impl IntoResponse {
    log::info!("query sys_operate_log_list params: {:?}", &item);
    let rb = &state.batis;

    let page = &PageRequest::new(item.page_no, item.page_size);
    let result = OperateLog::select_page(rb, page).await;

    let mut sys_operate_log_list_data: Vec<OperateLogListDataResp> = Vec::new();
    match result {
        Ok(d) => {
            let total = d.total;

            for x in d.records {
                sys_operate_log_list_data.push(OperateLogListDataResp {
                    id: x.id.unwrap(),                                        //日志主键
                    title: x.title.unwrap_or_default(),                       //模块标题
                    business_type: x.business_type.unwrap_or_default(), //业务类型（0其它 1新增 2修改 3删除）
                    method: x.method.unwrap_or_default(),               //方法名称
                    request_method: x.request_method.unwrap_or_default(), //请求方式
                    operator_type: x.operator_type.unwrap_or_default(), //操作类别（0其它 1后台用户 2手机端用户）
                    operate_name: x.operate_name.unwrap_or_default(),   //操作人员
                    dept_name: x.dept_name.unwrap_or_default(),         //部门名称
                    operate_url: x.operate_url.unwrap_or_default(),     //请求URL
                    operate_ip: x.operate_ip.unwrap_or_default(),       //主机地址
                    operate_location: x.operate_location.unwrap_or_default(), //操作地点
                    operate_param: x.operate_param.unwrap_or_default(), //请求参数
                    json_result: x.json_result.unwrap_or_default(),     //返回参数
                    status: x.status.unwrap_or_default(),               //操作状态(0:异常,正常)
                    error_msg: x.error_msg.unwrap_or_default(),         //错误消息
                    operate_time: x.operate_time.unwrap().0.to_string(), //操作时间
                    cost_time: x.cost_time.unwrap_or_default(),         //消耗时间
                })
            }

            BaseResponse::ok_result_page(sys_operate_log_list_data, total)
        }
        Err(err) => BaseResponse::err_result_page(OperateLogListDataResp::new(), err.to_string()),
    }
}
