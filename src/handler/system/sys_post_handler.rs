use crate::service::system::sys_post_service::PostService;
use crate::vo::system::sys_post_vo::*;
use crate::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use log::info;
use std::sync::Arc;
/*
 *添加岗位信息表
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn add_sys_post(State(state): State<Arc<AppState>>, Json(item): Json<PostReq>) -> impl IntoResponse {
    info!("add sys_post params: {:?}", &item);

    PostService::add_sys_post(state, item).await
}

/*
 *删除岗位信息表
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn delete_sys_post(State(state): State<Arc<AppState>>, Json(item): Json<DeletePostReq>) -> impl IntoResponse {
    info!("delete sys_post params: {:?}", &item);

    PostService::delete_sys_post(state, item).await
}

/*
 *更新岗位信息表
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn update_sys_post(State(state): State<Arc<AppState>>, Json(item): Json<PostReq>) -> impl IntoResponse {
    info!("update sys_post params: {:?}", &item);

    PostService::update_sys_post(state, item).await
}

/*
 *更新岗位信息表状态
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn update_sys_post_status(State(state): State<Arc<AppState>>, Json(item): Json<UpdatePostStatusReq>) -> impl IntoResponse {
    info!("update sys_post_status params: {:?}", &item);

    PostService::update_sys_post_status(state, item).await
}

/*
 *查询岗位信息表详情
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn query_sys_post_detail(State(state): State<Arc<AppState>>, Json(item): Json<QueryPostDetailReq>) -> impl IntoResponse {
    info!("query sys_post_detail params: {:?}", &item);

    PostService::query_sys_post_detail(state, item).await
}

/*
 *查询岗位信息表列表
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn query_sys_post_list(State(state): State<Arc<AppState>>, Json(item): Json<QueryPostListReq>) -> impl IntoResponse {
    info!("query sys_post_list params: {:?}", &item);

    PostService::query_sys_post_list(state, item).await
}
