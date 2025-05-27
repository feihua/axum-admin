use crate::common::result::BaseResponse;
use crate::model::system::sys_operate_log_model::{clean_operate_log, OperateLog};
use crate::utils::time_util::time_to_string;
use crate::vo::system::sys_operate_log_vo::*;
use crate::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use rbatis::plugin::page::PageRequest;
use rbs::value;
use std::sync::Arc;

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

    OperateLog::delete_by_map(rb, value! {"id": &item.ids}).await?;
    BaseResponse::<String>::ok_result()
}

/*
 *清空操作日志记录
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn clean_sys_operate_log(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    log::info!("clean sys_operate_log");
    let rb = &state.batis;

    clean_operate_log(rb).await?;

    BaseResponse::<String>::ok_result()
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

    match OperateLog::select_by_id(rb, &item.id).await? {
        None => BaseResponse::<QueryOperateLogDetailResp>::err_result_data(
            QueryOperateLogDetailResp::new(),
            "操作日志不存在",
        ),
        Some(x) => {
            let sys_operate_log = QueryOperateLogDetailResp {
                id: x.id,                                     //日志主键
                title: x.title,                               //模块标题
                business_type: x.business_type,               //业务类型（0其它 1新增 2修改 3删除）
                method: x.method,                             //方法名称
                request_method: x.request_method,             //请求方式
                operator_type: x.operator_type, //操作类别（0其它 1后台用户 2手机端用户）
                operate_name: x.operate_name,   //操作人员
                dept_name: x.dept_name,         //部门名称
                operate_url: x.operate_url,     //请求URL
                operate_ip: x.operate_ip,       //主机地址
                operate_location: x.operate_location, //操作地点
                operate_param: x.operate_param, //请求参数
                json_result: x.json_result,     //返回参数
                status: x.status,               //操作状态(0:异常,正常)
                error_msg: x.error_msg,         //错误消息
                operate_time: time_to_string(x.operate_time), //操作时间
                cost_time: x.cost_time,         //消耗时间
            };

            BaseResponse::<QueryOperateLogDetailResp>::ok_result_data(sys_operate_log)
        }
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

    let page = &PageRequest::new(item.page_no, item.page_size);
    let d = OperateLog::select_page_by_name(rb, page, &item).await?;

    let mut list: Vec<OperateLogListDataResp> = Vec::new();

    let total = d.total;

    for x in d.records {
        list.push(OperateLogListDataResp {
            id: x.id,                                     //日志主键
            title: x.title,                               //模块标题
            business_type: x.business_type,               //业务类型（0其它 1新增 2修改 3删除）
            method: x.method,                             //方法名称
            request_method: x.request_method,             //请求方式
            operator_type: x.operator_type,               //操作类别（0其它 1后台用户 2手机端用户）
            operate_name: x.operate_name,                 //操作人员
            dept_name: x.dept_name,                       //部门名称
            operate_url: x.operate_url,                   //请求URL
            operate_ip: x.operate_ip,                     //主机地址
            operate_location: x.operate_location,         //操作地点
            operate_param: x.operate_param,               //请求参数
            json_result: x.json_result,                   //返回参数
            status: x.status,                             //操作状态(0:异常,正常)
            error_msg: x.error_msg,                       //错误消息
            operate_time: time_to_string(x.operate_time), //操作时间
            cost_time: x.cost_time,                       //消耗时间
        })
    }

    BaseResponse::ok_result_page(list, total)
}
