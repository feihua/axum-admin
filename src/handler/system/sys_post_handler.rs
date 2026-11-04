use crate::AppState;
use crate::common::result::{ok_result_data, ok_result_page};
use crate::service::system::sys_post_service::PostService;
use crate::vo::system::sys_post_vo::*;
use axum::Json;
use axum::extract::State;
use axum::response::IntoResponse;
use axum_valid::Valid;
use std::sync::Arc;

use print_log_macro::log;
/*
 *添加岗位信息表
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
#[log]
pub async fn add_sys_post(State(state): State<Arc<AppState>>, Valid(Json(item)): Valid<Json<PostReq>>) -> impl IntoResponse {
    PostService::add_sys_post(state, item).await.map(|last_insert_id| ok_result_data(last_insert_id))?
}

/*
 *删除岗位信息表
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
#[log]
pub async fn delete_sys_post(State(state): State<Arc<AppState>>, Valid(Json(item)): Valid<Json<DeletePostReq>>) -> impl IntoResponse {
    PostService::delete_sys_post(state, item).await.map(|rows_affected| ok_result_data(rows_affected))?
}

/*
 *更新岗位信息表
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
#[log]
pub async fn update_sys_post(State(state): State<Arc<AppState>>, Valid(Json(item)): Valid<Json<PostReq>>) -> impl IntoResponse {
    PostService::update_sys_post(state, item).await.map(|rows_affected| ok_result_data(rows_affected))?
}

/*
 *更新岗位信息表状态
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
#[log]
pub async fn update_sys_post_status(State(state): State<Arc<AppState>>, Valid(Json(item)): Valid<Json<UpdatePostStatusReq>>) -> impl IntoResponse {
    PostService::update_sys_post_status(state, item).await.map(|rows_affected| ok_result_data(rows_affected))?
}

/*
 *查询岗位信息表详情
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
#[log]
pub async fn query_sys_post_detail(State(state): State<Arc<AppState>>, Valid(Json(item)): Valid<Json<QueryPostDetailReq>>) -> impl IntoResponse {
    PostService::query_sys_post_detail(state, item).await.map(|record| ok_result_data(record))?
}

/*
 *查询岗位信息表列表
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
#[log]
pub async fn query_sys_post_list(State(state): State<Arc<AppState>>, Valid(Json(item)): Valid<Json<QueryPostListReq>>) -> impl IntoResponse {
    PostService::query_sys_post_list(state, item).await.map(|(list, total)| ok_result_page(list, total))?
}
