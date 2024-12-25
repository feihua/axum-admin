use crate::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use rbatis::plugin::page::PageRequest;
use rbatis::rbdc::datetime::DateTime;
use rbs::to_value;
use std::sync::Arc;

use crate::common::result::BaseResponse;
use crate::model::system::sys_notice_model::Notice;
use crate::vo::system::sys_notice_vo::*;
use crate::vo::system::*;

/*
 *添加通知公告表
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
pub async fn add_sys_notice(
    State(state): State<Arc<AppState>>,
    Json(item): Json<AddNoticeReq>,
) -> impl IntoResponse {
    log::info!("add sys_notice params: {:?}", &item);
    let rb = &state.batis;

    let sys_notice = Notice {
        id: None,                            //公告ID
        notice_title: item.notice_title,     //公告标题
        notice_type: item.notice_type,       //公告类型（1:通知,2:公告）
        notice_content: item.notice_content, //公告内容
        status: item.status,                 //公告状态（0:关闭,1:正常 ）
        remark: item.remark,                 //备注
        create_time: None,                   //创建时间
        update_time: None,                   //修改时间
    };

    let result = Notice::insert(rb, &sys_notice).await;

    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

/*
 *删除通知公告表
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
pub async fn delete_sys_notice(
    State(state): State<Arc<AppState>>,
    Json(item): Json<DeleteNoticeReq>,
) -> impl IntoResponse {
    log::info!("delete sys_notice params: {:?}", &item);
    let rb = &state.batis;

    let result = Notice::delete_in_column(rb, "id", &item.ids).await;

    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

/*
 *更新通知公告表
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
pub async fn update_sys_notice(
    State(state): State<Arc<AppState>>,
    Json(item): Json<UpdateNoticeReq>,
) -> impl IntoResponse {
    log::info!("update sys_notice params: {:?}", &item);
    let rb = &state.batis;

    let sys_notice = Notice {
        id: Some(item.id),                   //公告ID
        notice_title: item.notice_title,     //公告标题
        notice_type: item.notice_type,       //公告类型（1:通知,2:公告）
        notice_content: item.notice_content, //公告内容
        status: item.status,                 //公告状态（0:关闭,1:正常 ）
        remark: item.remark,                 //备注
        create_time: None,                   //创建时间
        update_time: None,                   //修改时间
    };

    let result = Notice::update_by_column(rb, &sys_notice, "id").await;

    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

/*
 *更新通知公告表状态
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
pub async fn update_sys_notice_status(
    State(state): State<Arc<AppState>>,
    Json(item): Json<UpdateNoticeStatusReq>,
) -> impl IntoResponse {
    log::info!("update sys_notice_status params: {:?}", &item);
    let rb = &state.batis;

    let update_sql = format!(
        "update sys_notice set status = ? where id in ({})",
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
 *查询通知公告表详情
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
pub async fn query_sys_notice_detail(
    State(state): State<Arc<AppState>>,
    Json(item): Json<QueryNoticeDetailReq>,
) -> impl IntoResponse {
    log::info!("query sys_notice_detail params: {:?}", &item);
    let rb = &state.batis;

    let result = Notice::select_by_id(rb, &item.id).await;

    match result {
        Ok(d) => {
            let x = d.unwrap();

            let sys_notice = QueryNoticeDetailResp {
                id: x.id.unwrap(),                      //公告ID
                notice_title: x.notice_title,           //公告标题
                notice_type: x.notice_type,             //公告类型（1:通知,2:公告）
                notice_content: x.notice_content,       //公告内容
                status: x.status,                       //公告状态（0:关闭,1:正常 ）
                remark: x.remark,                       //备注
                create_time: x.create_time.to_string(), //创建时间
                update_time: x.update_time.to_string(), //修改时间
            };

            BaseResponse::<QueryNoticeDetailResp>::ok_result_data(sys_notice)
        }
        Err(err) => BaseResponse::<QueryNoticeDetailResp>::err_result_data(
            QueryNoticeDetailResp::new(),
            err.to_string(),
        ),
    }
}

/*
 *查询通知公告表列表
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
pub async fn query_sys_notice_list(
    State(state): State<Arc<AppState>>,
    Json(item): Json<QueryNoticeListReq>,
) -> impl IntoResponse {
    log::info!("query sys_notice_list params: {:?}", &item);
    let rb = &state.batis;

    let page = &PageRequest::new(item.page_no, item.page_size);
    let result = Notice::select_page(rb, page).await;

    let mut sys_notice_list_data: Vec<NoticeListDataResp> = Vec::new();
    match result {
        Ok(d) => {
            let total = d.total;

            for x in d.records {
                sys_notice_list_data.push(NoticeListDataResp {
                    id: x.id.unwrap(),                                 //公告ID
                    notice_title: x.notice_title,                      //公告标题
                    notice_type: x.notice_type,                        //公告类型（1:通知,2:公告）
                    notice_content: x.notice_content,                  //公告内容
                    status: x.status,                                  //公告状态（0:关闭,1:正常 ）
                    remark: x.remark,                                  //备注
                    create_time: x.create_time.unwrap().0.to_string(), //创建时间
                    update_time: x.update_time.unwrap().0.to_string(), //修改时间
                })
            }

            BaseResponse::ok_result_page(sys_notice_list_data, total)
        }
        Err(err) => BaseResponse::err_result_page(NoticeListDataResp::new(), err.to_string()),
    }
}
