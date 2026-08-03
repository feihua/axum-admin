use crate::common::error::AppError;
use crate::common::error::AppError::BusinessError;
use crate::model::system::sys_login_log_model::{clean_login_log, LoginLog};
use crate::vo::system::sys_login_log_vo::*;
use crate::AppState;
use rbatis::plugin::page::PageRequest;
use std::sync::Arc;

pub struct LoginLogService;
impl LoginLogService {
    /*
     *删除系统访问记录
     *author：刘飞华
     *date：2024/12/25 11:36:48
     */
    pub async fn delete_sys_login_log(state: Arc<AppState>, item: DeleteLoginLogReq) -> Result<u64, AppError> {
        let rb = &state.batis;
        LoginLog::delete(rb, item.ids).await.map(|x| Ok(x))?
    }

    /*
     *清空系统登录日志
     *author：刘飞华
     *date：2024/12/25 11:36:48
     */
    pub async fn clean_sys_login_log(state: Arc<AppState>) -> Result<i64, AppError> {
        let rb = &state.batis;
        clean_login_log(rb).await.map(|x| Ok(x.unwrap_or_default()))?
    }

    /*
     *查询系统访问记录详情
     *author：刘飞华
     *date：2024/12/25 11:36:48
     */
    pub async fn query_sys_login_log_detail(state: Arc<AppState>, item: QueryLoginLogDetailReq) -> Result<LoginLogResp, AppError> {
        let rb = &state.batis;
        let result = LoginLog::select_by_id(rb, &item.id).await?;

        match result {
            None => Err(BusinessError("系统访问记录不存在")),
            Some(x) => Ok(x.into()),
        }
    }

    /*
     *查询系统访问记录列表
     *author：刘飞华
     *date：2024/12/25 11:36:48
     */
    pub async fn query_sys_login_log_list(state: Arc<AppState>, item: QueryLoginLogListReq) -> Result<(Vec<LoginLogResp>, u64), AppError> {
        let rb = &state.batis;
        let page = &PageRequest::new(item.page_no, item.page_size);

        LoginLog::select_by_page(rb, page, &item)
            .await
            .map(|x| Ok((x.records.into_iter().map(|x| x.into()).collect::<Vec<LoginLogResp>>(), x.total)))?
    }
}
