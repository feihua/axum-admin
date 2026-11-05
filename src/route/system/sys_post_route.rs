use crate::AppState;
use crate::handler::system::sys_post_handler;
use axum::Router;
use route_macro::routes;
use std::sync::Arc;
/*
 *构建岗位信息表路由
 *author：刘飞华
 *date：2024/12/25 14:07:37
 */
pub fn build_sys_post_route() -> Router<Arc<AppState>> {
    routes! {
        "/system/post/addPost"=>post(sys_post_handler::add_sys_post),
        "/system/post/deletePost"=>post(sys_post_handler::delete_sys_post),
        "/system/post/updatePost"=>post(sys_post_handler::update_sys_post),
        "/system/post/updatePostStatus"=>post(sys_post_handler::update_sys_post_status),
        "/system/post/queryPostDetail"=>post(sys_post_handler::query_sys_post_detail),
        "/system/post/queryPostList"=>post(sys_post_handler::query_sys_post_list),
    }
}
