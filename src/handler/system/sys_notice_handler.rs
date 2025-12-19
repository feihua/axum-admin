use crate::common::error::AppError;
use crate::common::result::{ok_result, ok_result_data, ok_result_page};
use crate::model::system::sys_notice_model::Notice;
use crate::vo::system::sys_notice_vo::*;
use crate::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use log::info;
use rbatis::plugin::page::PageRequest;
use rbs::value;
use std::sync::Arc;
/*
 *添加通知公告表
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn add_sys_notice(State(state): State<Arc<AppState>>, Json(mut item): Json<NoticeReq>) -> impl IntoResponse {
    info!("add sys_notice params: {:?}", &item);
    let rb = &state.batis;

    if Notice::select_by_title(rb, &item.notice_title).await?.is_some() {
        return Err(AppError::BusinessError("公告标题已存在"));
    };

    item.id = None;
    Notice::insert(rb, &Notice::from(item)).await.map(|_| ok_result())?
}

/*
 *删除通知公告表
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn delete_sys_notice(State(state): State<Arc<AppState>>, Json(item): Json<DeleteNoticeReq>) -> impl IntoResponse {
    info!("delete sys_notice params: {:?}", &item);
    let rb = &state.batis;

    Notice::delete_by_map(rb, value! {"id": &item.ids}).await.map(|_| ok_result())?
}

/*
 *更新通知公告表
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn update_sys_notice(State(state): State<Arc<AppState>>, Json(item): Json<NoticeReq>) -> impl IntoResponse {
    info!("update sys_notice params: {:?}", &item);
    let rb = &state.batis;

    let id = item.id;
    if id.is_none() {
        return Err(AppError::BusinessError("主键不能为空"));
    }

    if Notice::select_by_id(rb, &id.unwrap_or_default()).await?.is_none() {
        return Err(AppError::BusinessError("通知公告表不存在"));
    }

    if let Some(x) = Notice::select_by_title(rb, &item.notice_title).await? {
        if x.id != id {
            return Err(AppError::BusinessError("公告标题已存在"));
        }
    }

    Notice::update_by_map(rb, &Notice::from(item), value! {"id": &id}).await.map(|_| ok_result())?
}

/*
 *更新通知公告表状态
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn update_sys_notice_status(State(state): State<Arc<AppState>>, Json(item): Json<UpdateNoticeStatusReq>) -> impl IntoResponse {
    info!("update sys_notice_status params: {:?}", &item);
    let rb = &state.batis;

    let update_sql = format!("update sys_notice set status = ? where id in ({})", item.ids.iter().map(|_| "?").collect::<Vec<&str>>().join(", "));

    let mut param = vec![value!(item.status)];
    param.extend(item.ids.iter().map(|&id| value!(id)));

    rb.exec(&update_sql, param).await.map(|_| ok_result())?
}

/*
 *查询通知公告表详情
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn query_sys_notice_detail(State(state): State<Arc<AppState>>, Json(item): Json<QueryNoticeDetailReq>) -> impl IntoResponse {
    info!("query sys_notice_detail params: {:?}", &item);
    let rb = &state.batis;

    Notice::select_by_id(rb, &item.id).await?.map_or_else(
        || Err(AppError::BusinessError("通知公告表不存在")),
        |x| {
            let notice: NoticeResp = x.into();
            ok_result_data(notice)
        },
    )
}

/*
 *查询通知公告表列表
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn query_sys_notice_list(State(state): State<Arc<AppState>>, Json(item): Json<QueryNoticeListReq>) -> impl IntoResponse {
    info!("query sys_notice_list params: {:?}", &item);
    let rb = &state.batis;

    let page = &PageRequest::new(item.page_no, item.page_size);

    Notice::select_sys_notice_list(rb, page, &item)
        .await
        .map(|x| ok_result_page(x.records.into_iter().map(|x| x.into()).collect::<Vec<NoticeResp>>(), x.total))?
}
