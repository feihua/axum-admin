use crate::AppState;
use axum::routing::post;
use axum::Router;
use std::sync::Arc;

/*
 *构建岗位信息表路由
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
pub fn build_sys_post_route() -> Router<Arc<AppState>> {
    Router::new()
        .route("/addPost", post(sys_post_handler::add_sys_post))
        .route("/deletePost", post(sys_post_handler::delete_sys_post))
        .route("/updatePost", post(sys_post_handler::update_sys_post))
        .route(
            "/updatePostStatus",
            post(sys_post_handler::update_sys_post_status),
        )
        .route(
            "/queryPostDetail",
            post(sys_post_handler::query_sys_post_detail),
        )
        .route(
            "/queryPostList",
            post(sys_post_handler::query_sys_post_list),
        )
    //记得在main.rs中添加路由build_sys_post_route()
}
