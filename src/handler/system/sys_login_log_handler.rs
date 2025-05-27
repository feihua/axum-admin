use crate::common::result::BaseResponse;
use crate::model::system::sys_login_log_model::{clean_login_log, LoginLog};
use crate::utils::time_util::time_to_string;
use crate::vo::system::sys_login_log_vo::*;
use crate::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use rbatis::plugin::page::PageRequest;
use rbs::value;
use std::sync::Arc;

/*
 *删除系统访问记录
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn delete_sys_login_log(
    State(state): State<Arc<AppState>>,
    Json(item): Json<DeleteLoginLogReq>,
) -> impl IntoResponse {
    log::info!("delete sys_login_log params: {:?}", &item);
    let rb = &state.batis;

    LoginLog::delete_by_map(rb, value! {"id": &item.ids}).await?;
    BaseResponse::<String>::ok_result()
}

/*
 *清空系统登录日志
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn clean_sys_login_log(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    log::info!("clean sys_login_log ");
    let rb = &state.batis;

    clean_login_log(rb).await?;

    BaseResponse::<String>::ok_result()
}

/*
 *查询系统访问记录详情
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn query_sys_login_log_detail(
    State(state): State<Arc<AppState>>,
    Json(item): Json<QueryLoginLogDetailReq>,
) -> impl IntoResponse {
    log::info!("query sys_login_log_detail params: {:?}", &item);
    let rb = &state.batis;

    match LoginLog::select_by_id(rb, &item.id).await? {
        None => BaseResponse::<QueryLoginLogDetailResp>::err_result_data(
            QueryLoginLogDetailResp::new(),
            "日志不存在",
        ),
        Some(x) => {
            let sys_login_log = QueryLoginLogDetailResp {
                id: x.id.unwrap_or_default(),             //访问ID
                login_name: x.login_name,                 //登录账号
                ipaddr: x.ipaddr,                         //登录IP地址
                login_location: x.login_location,         //登录地点
                platform: x.platform,                     //平台信息
                browser: x.browser,                       //浏览器类型
                version: x.version,                       //浏览器版本
                os: x.os,                                 //操作系统
                arch: x.arch,                             //体系结构信息
                engine: x.engine,                         //渲染引擎信息
                engine_details: x.engine_details,         //渲染引擎详细信息
                extra: x.extra,                           //其他信息（可选）
                status: x.status,                         //登录状态(0:失败,1:成功)
                msg: x.msg,                               //提示消息
                login_time: time_to_string(x.login_time), //访问时间
            };

            BaseResponse::<QueryLoginLogDetailResp>::ok_result_data(sys_login_log)
        }
    }
}

/*
 *查询系统访问记录列表
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn query_sys_login_log_list(
    State(state): State<Arc<AppState>>,
    Json(item): Json<QueryLoginLogListReq>,
) -> impl IntoResponse {
    log::info!("query sys_login_log_list params: {:?}", &item);
    let rb = &state.batis;

    let page = &PageRequest::new(item.page_no, item.page_size);
    let d = LoginLog::select_login_log_list(rb, page, &item).await?;

    let mut list: Vec<LoginLogListDataResp> = Vec::new();

    let total = d.total;

    for x in d.records {
        list.push(LoginLogListDataResp {
            id: x.id.unwrap_or_default(),             //访问ID
            login_name: x.login_name,                 //登录账号
            ipaddr: x.ipaddr,                         //登录IP地址
            login_location: x.login_location,         //登录地点
            platform: x.platform,                     //平台信息
            browser: x.browser,                       //浏览器类型
            version: x.version,                       //浏览器版本
            os: x.os,                                 //操作系统
            arch: x.arch,                             //体系结构信息
            engine: x.engine,                         //渲染引擎信息
            engine_details: x.engine_details,         //渲染引擎详细信息
            extra: x.extra,                           //其他信息（可选）
            status: x.status,                         //登录状态(0:失败,1:成功)
            msg: x.msg,                               //提示消息
            login_time: time_to_string(x.login_time), //访问时间
        })
    }

    BaseResponse::ok_result_page(list, total)
}
