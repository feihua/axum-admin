use crate::common::error::AppError;
use crate::common::result::{ok_result, ok_result_data, ok_result_page};
use crate::model::system::sys_post_model::Post;
use crate::model::system::sys_user_post_model::UserPost;
use crate::vo::system::sys_post_vo::*;
use crate::AppState;
use axum::response::IntoResponse;
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
    pub async fn add_sys_post(state: Arc<AppState>, mut item: PostReq) -> impl IntoResponse {
        let rb = &state.batis;
        if Post::select_by_map(rb, value! {"post_name": &item.post_name}).await?.len() > 0 {
            return Err(AppError::BusinessError("岗位名称已存在"));
        }

        if Post::select_by_map(rb, value! {"post_code": &item.post_code}).await?.len() > 0 {
            return Err(AppError::BusinessError("岗位编码已存在"));
        }

        item.id = None;
        Post::insert(rb, &Post::from(item)).await.map(|_| ok_result())?
    }

    /*
     *删除岗位信息表
     *author：刘飞华
     *date：2024/12/25 11:36:48
     */
    pub async fn delete_sys_post(state: Arc<AppState>, item: DeletePostReq) -> impl IntoResponse {
        let rb = &state.batis;
        let ids = item.ids;
        for id in &ids {
            if UserPost::count_user_post_by_id(rb, id).await? > 0 {
                return Err(AppError::BusinessError("已分配,不能删除"));
            }
        }

        Post::delete_by_map(rb, value! {"id": ids}).await.map(|_| ok_result())?
    }

    /*
     *更新岗位信息表
     *author：刘飞华
     *date：2024/12/25 11:36:48
     */
    pub async fn update_sys_post(state: Arc<AppState>, item: PostReq) -> impl IntoResponse {
        let rb = &state.batis;
        let id = item.id;

        if id.is_none() {
            return Err(AppError::BusinessError("主键不能为空"));
        }

        if Post::select_by_id(rb, &id.unwrap_or_default()).await?.is_none() {
            return Err(AppError::BusinessError("岗位不存在"));
        }

        if Post::select_by_map(rb, value! {"post_name": &item.post_name, "id !=": id}).await?.len() > 0 {
            return Err(AppError::BusinessError("岗位名称已存在"));
        }

        if Post::select_by_map(rb, value! {"post_code": &item.post_code, "id !=": id}).await?.len() > 0 {
            return Err(AppError::BusinessError("岗位编码已存在"));
        }

        Post::update_by_map(rb, &Post::from(item), value! {"id": &id}).await.map(|_| ok_result())?
    }

    /*
     *更新岗位信息表状态
     *author：刘飞华
     *date：2024/12/25 11:36:48
     */
    pub async fn update_sys_post_status(state: Arc<AppState>, item: UpdatePostStatusReq) -> impl IntoResponse {
        let rb = &state.batis;
        let update_sql = format!("update sys_post set status = ? where id in ({})", item.ids.iter().map(|_| "?").collect::<Vec<&str>>().join(", "));

        let mut param = vec![value!(item.status)];
        param.extend(item.ids.iter().map(|&id| value!(id)));
        rb.exec(&update_sql, param).await.map(|_| ok_result())?
    }

    /*
     *查询岗位信息表详情
     *author：刘飞华
     *date：2024/12/25 11:36:48
     */
    pub async fn query_sys_post_detail(state: Arc<AppState>, item: QueryPostDetailReq) -> impl IntoResponse {
        let rb = &state.batis;
        Post::select_by_id(rb, &item.id).await?.map_or_else(
            || Err(AppError::BusinessError("岗位不存在")),
            |x| {
                let data: PostResp = x.into();
                ok_result_data(data)
            },
        )
    }

    /*
     *查询岗位信息表列表
     *author：刘飞华
     *date：2024/12/25 11:36:48
     */
    pub async fn query_sys_post_list(state: Arc<AppState>, item: QueryPostListReq) -> impl IntoResponse {
        let rb = &state.batis;
        let page = &PageRequest::new(item.page_no, item.page_size);

        Post::select_by_page(rb, page, &item)
            .await
            .map(|x| ok_result_page(x.records.into_iter().map(|x| x.into()).collect::<Vec<PostResp>>(), x.total))?
    }
}
