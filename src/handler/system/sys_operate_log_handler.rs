use crate::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use rbatis::plugin::page::PageRequest;
use std::sync::Arc;

use crate::common::result::BaseResponse;
use crate::model::system::sys_operate_log_model::OperateLog;
use crate::vo::system::sys_operate_log_vo::*;

/*
 *删除操作日志记录
 *author：刘飞华
 *date：2024/12/25 11:36:48
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
 *查询操作日志记录详情
 *author：刘飞华
 *date：2024/12/25 11:36:48
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
                operate_time: x.operate_time.unwrap().0.to_string(), //操作时间
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
 *date：2024/12/25 11:36:48
 */
pub async fn query_sys_operate_log_list(
    State(state): State<Arc<AppState>>,
    Json(item): Json<QueryOperateLogListReq>,
) -> impl IntoResponse {
    log::info!("query sys_operate_log_list params: {:?}", &item);
    let rb = &state.batis;
    //let title = item.title.as_deref().unwrap_or_default(); //模块标题
    //let business_type = item.business_type.unwrap_or(2); //业务类型（0其它 1新增 2修改 3删除）
    //let method = item.method.as_deref().unwrap_or_default(); //方法名称
    //let request_method = item.request_method.as_deref().unwrap_or_default(); //请求方式
    //let operator_type = item.operator_type.unwrap_or(2); //操作类别（0其它 1后台用户 2手机端用户）
    //let operate_name = item.operate_name.as_deref().unwrap_or_default(); //操作人员
    //let dept_name = item.dept_name.as_deref().unwrap_or_default(); //部门名称
    //let operate_url = item.operate_url.as_deref().unwrap_or_default(); //请求URL
    //let operate_ip = item.operate_ip.as_deref().unwrap_or_default(); //主机地址
    //let operate_location = item.operate_location.as_deref().unwrap_or_default(); //操作地点
    //let operate_param = item.operate_param.as_deref().unwrap_or_default(); //请求参数
    //let json_result = item.json_result.as_deref().unwrap_or_default(); //返回参数
    //let status = item.status.unwrap_or(2); //操作状态(0:异常,正常)
    //let error_msg = item.error_msg.as_deref().unwrap_or_default(); //错误消息
    //let operate_time = item.operate_time.unwrap_or(2); //操作时间
    //let cost_time = item.cost_time.unwrap_or(2); //消耗时间

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
