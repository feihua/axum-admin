use crate::common::error::AppError;
use crate::common::error::AppError::BusinessError;
use crate::model::system::sys_operate_log_model::{clean_operate_log, OperateLog};
use crate::vo::system::sys_operate_log_vo::*;
use crate::AppState;
use curd_trait::BaseCurd;
use rbatis::plugin::page::PageRequest;
use std::sync::Arc;

pub struct OperateLogService;
impl OperateLogService {
    /*
     *删除操作日志记录
     *author：刘飞华
     *date：2024/12/25 11:36:48
     */
    pub async fn delete_sys_operate_log(state: Arc<AppState>, item: DeleteOperateLogReq) -> Result<u64, AppError> {
        let rb = &state.batis;
        OperateLog::delete(rb, item.ids).await.map(|x| Ok(x))?
    }

    /*
     *清空操作日志记录
     *author：刘飞华
     *date：2024/12/25 11:36:48
     */
    pub async fn clean_sys_operate_log(state: Arc<AppState>) -> Result<i64, AppError> {
        let rb = &state.batis;
        clean_operate_log(rb).await.map(|x| Ok(x.unwrap_or_default()))?
    }

    /*
     *查询操作日志记录详情
     *author：刘飞华
     *date：2024/12/25 11:36:48
     */
    pub async fn query_sys_operate_log_detail(state: Arc<AppState>, item: QueryOperateLogDetailReq) -> Result<OperateLogResp, AppError> {
        let rb = &state.batis;
        let result = OperateLog::select_by_id(rb, &item.id).await?;

        match result {
            None => Err(BusinessError("操作日志记录不存在")),
            Some(x) => Ok(x.into()),
        }
    }
    /*
     *查询操作日志记录列表
     *author：刘飞华
     *date：2024/12/25 11:36:48
     */
    pub async fn query_sys_operate_log_list(state: Arc<AppState>, item: QueryOperateLogListReq) -> Result<(Vec<OperateLogResp>, u64), AppError> {
        let rb = &state.batis;
        let page = &PageRequest::new(item.page_no, item.page_size);
        OperateLog::select_by_page(rb, page, &item)
            .await
            .map(|x| Ok((x.records.into_iter().map(|x| x.into()).collect::<Vec<OperateLogResp>>(), x.total)))?
    }
}
