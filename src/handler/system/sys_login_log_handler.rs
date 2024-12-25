use crate::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use rbatis::plugin::page::PageRequest;
use rbatis::rbdc::datetime::DateTime;
use rbs::to_value;
use std::sync::Arc;

use crate::common::result::BaseResponse;
use crate::model::system::sys_login_log_model::LoginLog;
use crate::vo::system::sys_login_log_vo::*;
use crate::vo::system::*;

/*
 *添加系统访问记录
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
pub async fn add_sys_login_log(
    State(state): State<Arc<AppState>>,
    Json(item): Json<AddLoginLogReq>,
) -> impl IntoResponse {
    log::info!("add sys_login_log params: {:?}", &item);
    let rb = &state.batis;

    let sys_login_log = LoginLog {
        id: None,                            //访问ID
        login_name: item.login_name,         //登录账号
        ipaddr: item.ipaddr,                 //登录IP地址
        login_location: item.login_location, //登录地点
        browser: item.browser,               //浏览器类型
        os: item.os,                         //操作系统
        status: item.status,                 //登录状态(0:失败,1:成功)
        msg: item.msg,                       //提示消息
        login_time: item.login_time,         //访问时间
    };

    let result = LoginLog::insert(rb, &sys_login_log).await;

    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

/*
 *删除系统访问记录
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
pub async fn delete_sys_login_log(
    State(state): State<Arc<AppState>>,
    Json(item): Json<DeleteLoginLogReq>,
) -> impl IntoResponse {
    log::info!("delete sys_login_log params: {:?}", &item);
    let rb = &state.batis;

    let result = LoginLog::delete_in_column(rb, "id", &item.ids).await;

    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

/*
 *更新系统访问记录
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
pub async fn update_sys_login_log(
    State(state): State<Arc<AppState>>,
    Json(item): Json<UpdateLoginLogReq>,
) -> impl IntoResponse {
    log::info!("update sys_login_log params: {:?}", &item);
    let rb = &state.batis;

    let sys_login_log = LoginLog {
        id: Some(item.id),                   //访问ID
        login_name: item.login_name,         //登录账号
        ipaddr: item.ipaddr,                 //登录IP地址
        login_location: item.login_location, //登录地点
        browser: item.browser,               //浏览器类型
        os: item.os,                         //操作系统
        status: item.status,                 //登录状态(0:失败,1:成功)
        msg: item.msg,                       //提示消息
        login_time: item.login_time,         //访问时间
    };

    let result = LoginLog::update_by_column(rb, &sys_login_log, "id").await;

    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

/*
 *更新系统访问记录状态
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
pub async fn update_sys_login_log_status(
    State(state): State<Arc<AppState>>,
    Json(item): Json<UpdateLoginLogStatusReq>,
) -> impl IntoResponse {
    log::info!("update sys_login_log_status params: {:?}", &item);
    let rb = &state.batis;

    let update_sql = format!(
        "update sys_login_log set status = ? where id in ({})",
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
 *查询系统访问记录详情
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
pub async fn query_sys_login_log_detail(
    State(state): State<Arc<AppState>>,
    Json(item): Json<QueryLoginLogDetailReq>,
) -> impl IntoResponse {
    log::info!("query sys_login_log_detail params: {:?}", &item);
    let rb = &state.batis;

    let result = LoginLog::select_by_id(rb, &item.id).await;

    match result {
        Ok(d) => {
            let x = d.unwrap();

            let sys_login_log = QueryLoginLogDetailResp {
                id: x.id.unwrap(),                    //访问ID
                login_name: x.login_name,             //登录账号
                ipaddr: x.ipaddr,                     //登录IP地址
                login_location: x.login_location,     //登录地点
                browser: x.browser,                   //浏览器类型
                os: x.os,                             //操作系统
                status: x.status,                     //登录状态(0:失败,1:成功)
                msg: x.msg,                           //提示消息
                login_time: x.login_time.to_string(), //访问时间
            };

            BaseResponse::<QueryLoginLogDetailResp>::ok_result_data(sys_login_log)
        }
        Err(err) => BaseResponse::<QueryLoginLogDetailResp>::err_result_data(
            QueryLoginLogDetailResp::new(),
            err.to_string(),
        ),
    }
}

/*
 *查询系统访问记录列表
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
pub async fn query_sys_login_log_list(
    State(state): State<Arc<AppState>>,
    Json(item): Json<QueryLoginLogListReq>,
) -> impl IntoResponse {
    log::info!("query sys_login_log_list params: {:?}", &item);
    let rb = &state.batis;

    let page = &PageRequest::new(item.page_no, item.page_size);
    let result = LoginLog::select_page(rb, page).await;

    let mut sys_login_log_list_data: Vec<LoginLogListDataResp> = Vec::new();
    match result {
        Ok(d) => {
            let total = d.total;

            for x in d.records {
                sys_login_log_list_data.push(LoginLogListDataResp {
                    id: x.id.unwrap(),                               //访问ID
                    login_name: x.login_name,                        //登录账号
                    ipaddr: x.ipaddr,                                //登录IP地址
                    login_location: x.login_location,                //登录地点
                    browser: x.browser,                              //浏览器类型
                    os: x.os,                                        //操作系统
                    status: x.status,                                //登录状态(0:失败,1:成功)
                    msg: x.msg,                                      //提示消息
                    login_time: x.login_time.unwrap().0.to_string(), //访问时间
                })
            }

            BaseResponse::ok_result_page(sys_login_log_list_data, total)
        }
        Err(err) => BaseResponse::err_result_page(LoginLogListDataResp::new(), err.to_string()),
    }
}
