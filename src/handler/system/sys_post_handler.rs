use crate::common::result::{ok_result_data, ok_result_page};
use crate::service::system::sys_post_service::PostService;
use crate::vo::system::sys_post_vo::*;
use crate::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use axum_valid::Valid;
use log::info;
use std::sync::Arc;
/*
 *添加岗位信息表
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn add_sys_post(State(state): State<Arc<AppState>>, Valid(Json(item)): Valid<Json<PostReq>>) -> impl IntoResponse {
    info!("add sys_post params: {:?}", &item);

    PostService::add_sys_post(state, item).await.map(|last_insert_id| ok_result_data(last_insert_id))?
}

/*
 *删除岗位信息表
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn delete_sys_post(State(state): State<Arc<AppState>>, Valid(Json(item)): Valid<Json<DeletePostReq>>) -> impl IntoResponse {
    info!("delete sys_post params: {:?}", &item);

    PostService::delete_sys_post(state, item).await.map(|rows_affected| ok_result_data(rows_affected))?
}

/*
 *更新岗位信息表
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn update_sys_post(State(state): State<Arc<AppState>>, Valid(Json(item)): Valid<Json<PostReq>>) -> impl IntoResponse {
    info!("update sys_post params: {:?}", &item);

    PostService::update_sys_post(state, item).await.map(|rows_affected| ok_result_data(rows_affected))?
}

/*
 *更新岗位信息表状态
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn update_sys_post_status(State(state): State<Arc<AppState>>, Valid(Json(item)): Valid<Json<UpdatePostStatusReq>>) -> impl IntoResponse {
    info!("update sys_post_status params: {:?}", &item);

    PostService::update_sys_post_status(state, item).await.map(|rows_affected| ok_result_data(rows_affected))?
}

/*
 *查询岗位信息表详情
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn query_sys_post_detail(State(state): State<Arc<AppState>>, Valid(Json(item)): Valid<Json<QueryPostDetailReq>>) -> impl IntoResponse {
    info!("query sys_post_detail params: {:?}", &item);

    PostService::query_sys_post_detail(state, item).await.map(|record| ok_result_data(record))?
}

/*
 *查询岗位信息表列表
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn query_sys_post_list(State(state): State<Arc<AppState>>, Valid(Json(item)): Valid<Json<QueryPostListReq>>) -> impl IntoResponse {
    info!("query sys_post_list params: {:?}", &item);

    PostService::query_sys_post_list(state, item).await.map(|(list, total)| ok_result_page(list, total))?
}
