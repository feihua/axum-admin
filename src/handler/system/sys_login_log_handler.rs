use crate::common::error::AppError;
use crate::common::result::{ok_result, ok_result_data, ok_result_page};
use crate::model::system::sys_login_log_model::{clean_login_log, LoginLog};
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
pub async fn delete_sys_login_log(State(state): State<Arc<AppState>>, Json(item): Json<DeleteLoginLogReq>) -> impl IntoResponse {
    log::info!("delete sys_login_log params: {:?}", &item);
    let rb = &state.batis;

    LoginLog::delete_by_map(rb, value! {"id": &item.ids}).await.map(|_| ok_result())?
}

/*
 *清空系统登录日志
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn clean_sys_login_log(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    log::info!("clean sys_login_log ");
    let rb = &state.batis;

    clean_login_log(rb).await.map(|_| ok_result())?
}

/*
 *查询系统访问记录详情
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn query_sys_login_log_detail(State(state): State<Arc<AppState>>, Json(item): Json<QueryLoginLogDetailReq>) -> impl IntoResponse {
    log::info!("query sys_login_log_detail params: {:?}", &item);
    let rb = &state.batis;

    LoginLog::select_by_id(rb, &item.id).await?.map_or_else(
        || Err(AppError::BusinessError("系统访问记录不存在")),
        |x| {
            let data: LoginLogResp = x.into();
            ok_result_data(data)
        },
    )
}

/*
 *查询系统访问记录列表
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn query_sys_login_log_list(State(state): State<Arc<AppState>>, Json(item): Json<QueryLoginLogListReq>) -> impl IntoResponse {
    log::info!("query sys_login_log_list params: {:?}", &item);
    let rb = &state.batis;

    let page = &PageRequest::new(item.page_no, item.page_size);

    LoginLog::select_login_log_list(rb, page, &item)
        .await
        .map(|x| ok_result_page(x.records.into_iter().map(|x| x.into()).collect::<Vec<LoginLogResp>>(), x.total))?
}
