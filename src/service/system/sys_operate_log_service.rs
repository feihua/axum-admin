use crate::common::error::AppError;
use crate::common::result::{ok_result, ok_result_data, ok_result_page};
use crate::model::system::sys_operate_log_model::{clean_operate_log, OperateLog};
use crate::vo::system::sys_operate_log_vo::*;
use crate::AppState;
use axum::response::IntoResponse;
use rbatis::plugin::page::PageRequest;
use rbs::value;
use std::sync::Arc;

pub struct OperateLogService;
impl OperateLogService {
    /*
     *删除操作日志记录
     *author：刘飞华
     *date：2024/12/25 11:36:48
     */
    pub async fn delete_sys_operate_log(state: Arc<AppState>, item: DeleteOperateLogReq) -> impl IntoResponse {
        let rb = &state.batis;
        OperateLog::delete_by_map(rb, value! {"id": item.ids}).await.map(|_| ok_result())?
    }

    /*
     *清空操作日志记录
     *author：刘飞华
     *date：2024/12/25 11:36:48
     */
    pub async fn clean_sys_operate_log(state: Arc<AppState>) -> impl IntoResponse {
        let rb = &state.batis;
        clean_operate_log(rb).await.map(|_| ok_result())?
    }

    /*
     *查询操作日志记录详情
     *author：刘飞华
     *date：2024/12/25 11:36:48
     */
    pub async fn query_sys_operate_log_detail(state: Arc<AppState>, item: QueryOperateLogDetailReq) -> impl IntoResponse {
        let rb = &state.batis;
        OperateLog::select_by_id(rb, &item.id).await?.map_or_else(
            || Err(AppError::BusinessError("操作日志不存在")),
            |x| {
                let data: OperateLogResp = x.into();
                ok_result_data(data)
            },
        )
    }
    /*
     *查询操作日志记录列表
     *author：刘飞华
     *date：2024/12/25 11:36:48
     */
    pub async fn query_sys_operate_log_list(state: Arc<AppState>, item: QueryOperateLogListReq) -> impl IntoResponse {
        let rb = &state.batis;
        let page = &PageRequest::new(item.page_no, item.page_size);
        OperateLog::select_by_page(rb, page, &item)
            .await
            .map(|x| ok_result_page(x.records.into_iter().map(|x| x.into()).collect::<Vec<OperateLogResp>>(), x.total))?
    }
}
