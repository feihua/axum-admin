use crate::handler::system::sys_post_handler;
use crate::AppState;
use axum::routing::post;
use axum::Router;
use std::sync::Arc;
/*
 *构建岗位信息表路由
 *author：刘飞华
 *date：2024/12/25 14:07:37
 */
pub fn build_sys_post_route() -> Router<Arc<AppState>> {
    Router::new()
        .route("/system/post/addPost", post(sys_post_handler::add_sys_post))
        .route("/system/post/deletePost", post(sys_post_handler::delete_sys_post))
        .route("/system/post/updatePost", post(sys_post_handler::update_sys_post))
        .route("/system/post/updatePostStatus", post(sys_post_handler::update_sys_post_status))
        .route("/system/post/queryPostDetail", post(sys_post_handler::query_sys_post_detail))
        .route("/system/post/queryPostList", post(sys_post_handler::query_sys_post_list))
    //记得在main.rs中添加路由build_sys_post_route()
}
