use crate::common::error::AppError;
use crate::common::result::{ok_result, ok_result_data, ok_result_page};
use crate::model::system::sys_notice_model::Notice;
use crate::utils::time_util::time_to_string;
use crate::vo::system::sys_notice_vo::*;
use crate::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use rbatis::plugin::page::PageRequest;
use rbs::value;
use std::sync::Arc;
/*
 *添加通知公告表
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn add_sys_notice(State(state): State<Arc<AppState>>, Json(item): Json<AddNoticeReq>) -> impl IntoResponse {
    log::info!("add sys_notice params: {:?}", &item);
    let rb = &state.batis;

    let title = item.notice_title;
    if Notice::select_by_title(rb, &title).await?.is_some() {
        return Err(AppError::BusinessError("公告标题已存在"));
    };

    let sys_notice = Notice {
        id: None,                                //公告ID
        notice_title: title,                     //公告标题
        notice_type: item.notice_type,           //公告类型（1:通知,2:公告）
        notice_content: item.notice_content,     //公告内容
        status: item.status,                     //公告状态（0:关闭,1:正常 ）
        remark: item.remark.unwrap_or_default(), //备注
        create_time: None,                       //创建时间
        update_time: None,                       //修改时间
    };

    Notice::insert(rb, &sys_notice).await?;
    ok_result()
}

/*
 *删除通知公告表
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn delete_sys_notice(State(state): State<Arc<AppState>>, Json(item): Json<DeleteNoticeReq>) -> impl IntoResponse {
    log::info!("delete sys_notice params: {:?}", &item);
    let rb = &state.batis;

    Notice::delete_by_map(rb, value! {"id": &item.ids}).await?;
    ok_result()
}

/*
 *更新通知公告表
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn update_sys_notice(State(state): State<Arc<AppState>>, Json(item): Json<UpdateNoticeReq>) -> impl IntoResponse {
    log::info!("update sys_notice params: {:?}", &item);
    let rb = &state.batis;

    if Notice::select_by_id(rb, &item.id).await?.is_none() {
        return Err(AppError::BusinessError("通知公告表不存在"));
    }

    if let Some(x) = Notice::select_by_title(rb, &item.notice_title).await? {
        if x.id.unwrap_or_default() != item.id {
            return Err(AppError::BusinessError("公告标题已存在"));
        }
    }

    let sys_notice = Notice {
        id: Some(item.id),                       //公告ID
        notice_title: item.notice_title,         //公告标题
        notice_type: item.notice_type,           //公告类型（1:通知,2:公告）
        notice_content: item.notice_content,     //公告内容
        status: item.status,                     //公告状态（0:关闭,1:正常 ）
        remark: item.remark.unwrap_or_default(), //备注
        create_time: None,                       //创建时间
        update_time: None,                       //修改时间
    };

    Notice::update_by_map(rb, &sys_notice, value! {"id": &item.id}).await?;
    ok_result()
}

/*
 *更新通知公告表状态
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn update_sys_notice_status(State(state): State<Arc<AppState>>, Json(item): Json<UpdateNoticeStatusReq>) -> impl IntoResponse {
    log::info!("update sys_notice_status params: {:?}", &item);
    let rb = &state.batis;

    let update_sql = format!("update sys_notice set status = ? where id in ({})", item.ids.iter().map(|_| "?").collect::<Vec<&str>>().join(", "));

    let mut param = vec![value!(item.status)];
    param.extend(item.ids.iter().map(|&id| value!(id)));

    rb.exec(&update_sql, param).await?;
    ok_result()
}

/*
 *查询通知公告表详情
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn query_sys_notice_detail(State(state): State<Arc<AppState>>, Json(item): Json<QueryNoticeDetailReq>) -> impl IntoResponse {
    log::info!("query sys_notice_detail params: {:?}", &item);
    let rb = &state.batis;

    match Notice::select_by_id(rb, &item.id).await? {
        None => Err(AppError::BusinessError("通知公告表不存在")),
        Some(x) => {
            let sys_notice = QueryNoticeDetailResp {
                id: x.id.unwrap_or_default(),               //公告ID
                notice_title: x.notice_title,               //公告标题
                notice_type: x.notice_type,                 //公告类型（1:通知,2:公告）
                notice_content: x.notice_content,           //公告内容
                status: x.status,                           //公告状态（0:关闭,1:正常 ）
                remark: x.remark,                           //备注
                create_time: time_to_string(x.create_time), //创建时间
                update_time: time_to_string(x.update_time), //修改时间
            };

            ok_result_data(sys_notice)
        }
    }
}

/*
 *查询通知公告表列表
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn query_sys_notice_list(State(state): State<Arc<AppState>>, Json(item): Json<QueryNoticeListReq>) -> impl IntoResponse {
    log::info!("query sys_notice_list params: {:?}", &item);
    let rb = &state.batis;

    let page = &PageRequest::new(item.page_no, item.page_size);

    let mut data: Vec<NoticeListDataResp> = Vec::new();
    let d = Notice::select_sys_notice_list(rb, page, &item).await?;
    let total = d.total;

    for x in d.records {
        data.push(NoticeListDataResp {
            id: x.id.unwrap_or_default(),               //公告ID
            notice_title: x.notice_title,               //公告标题
            notice_type: x.notice_type,                 //公告类型（1:通知,2:公告）
            notice_content: x.notice_content,           //公告内容
            status: x.status,                           //公告状态（0:关闭,1:正常 ）
            remark: x.remark,                           //备注
            create_time: time_to_string(x.create_time), //创建时间
            update_time: time_to_string(x.update_time), //修改时间
        })
    }

    ok_result_page(data, total)
}
