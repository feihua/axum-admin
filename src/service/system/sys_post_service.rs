use crate::common::error::AppError;
use crate::common::error::AppError::BusinessError;
use crate::model::system::sys_post_model::Post;
use crate::model::system::sys_user_post_model::UserPost;
use crate::vo::system::sys_post_vo::*;
use crate::AppState;
use curd_trait::BaseCurd;
use rbatis::plugin::page::PageRequest;
use rbs::value;
use std::sync::Arc;

pub struct PostService;
impl PostService {
    /*
     *添加岗位信息表
     *author：刘飞华
     *date：2024/12/25 11:36:48
     */
    pub async fn add_sys_post(state: Arc<AppState>, mut item: PostReq) -> Result<u64, AppError> {
        let rb = &state.batis;
        if Post::select_by_map(rb, value! {"post_name": &item.post_name}).await?.len() > 0 {
            return Err(BusinessError("岗位名称已存在"));
        }

        if Post::select_by_map(rb, value! {"post_code": &item.post_code}).await?.len() > 0 {
            return Err(BusinessError("岗位编码已存在"));
        }

        item.id = None;
        Post::add(rb, &Post::from(item)).await.map(|id| Ok(id.unwrap_or_default()))?
    }

    /*
     *删除岗位信息表
     *author：刘飞华
     *date：2024/12/25 11:36:48
     */
    pub async fn delete_sys_post(state: Arc<AppState>, item: DeletePostReq) -> Result<u64, AppError> {
        let rb = &state.batis;
        let ids = item.ids;
        for id in &ids {
            if UserPost::count_user_post_by_id(rb, id).await? > 0 {
                return Err(BusinessError("已分配,不能删除"));
            }
        }

        Post::delete(rb, ids).await.map(|x| Ok(x))?
    }

    /*
     *更新岗位信息表
     *author：刘飞华
     *date：2024/12/25 11:36:48
     */
    pub async fn update_sys_post(state: Arc<AppState>, item: PostReq) -> Result<u64, AppError> {
        let rb = &state.batis;
        let id = item.id;

        if id.is_none() {
            return Err(BusinessError("主键不能为空"));
        }

        if Post::select_by_id(rb, &id.unwrap_or_default()).await?.is_none() {
            return Err(BusinessError("岗位不存在"));
        }

        if Post::select_by_map(rb, value! {"post_name": &item.post_name, "id !=": id}).await?.len() > 0 {
            return Err(BusinessError("岗位名称已存在"));
        }

        if Post::select_by_map(rb, value! {"post_code": &item.post_code, "id !=": id}).await?.len() > 0 {
            return Err(BusinessError("岗位编码已存在"));
        }

        Post::update(rb, &Post::from(item), id.unwrap_or_default()).await.map(|x| Ok(x))?
    }

    /*
     *更新岗位信息表状态
     *author：刘飞华
     *date：2024/12/25 11:36:48
     */
    pub async fn update_sys_post_status(state: Arc<AppState>, item: UpdatePostStatusReq) -> Result<u64, AppError> {
        let rb = &state.batis;
        Post::update_status(rb, item.ids, item.status).await.map(|x| Ok(x))?
    }

    /*
     *查询岗位信息表详情
     *author：刘飞华
     *date：2024/12/25 11:36:48
     */
    pub async fn query_sys_post_detail(state: Arc<AppState>, item: QueryPostDetailReq) -> Result<PostResp, AppError> {
        let rb = &state.batis;
        let result = Post::select_by_id(rb, &item.id).await?;

        match result {
            None => Err(BusinessError("岗位信息表不存在")),
            Some(x) => Ok(x.into()),
        }
    }

    /*
     *查询岗位信息表列表
     *author：刘飞华
     *date：2024/12/25 11:36:48
     */
    pub async fn query_sys_post_list(state: Arc<AppState>, item: QueryPostListReq) -> Result<(Vec<PostResp>, u64), AppError> {
        let rb = &state.batis;
        let page = &PageRequest::new(item.page_no, item.page_size);

        Post::select_by_page(rb, page, &item)
            .await
            .map(|x| Ok((x.records.into_iter().map(|x| x.into()).collect::<Vec<PostResp>>(), x.total)))?
    }
}
