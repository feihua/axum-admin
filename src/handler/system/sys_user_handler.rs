use crate::service::system::sys_user_service::UserService;
use crate::vo::system::sys_user_vo::*;
use crate::AppState;
use axum::extract::State;
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::Json;
use log::info;
use std::sync::Arc;
/*
 *添加用户信息
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn add_sys_user(State(state): State<Arc<AppState>>, Json(item): Json<UserReq>) -> impl IntoResponse {
    info!("add sys_user params: {:?}", &item);

    UserService::add_sys_user(state, item).await
}

/*
 *删除用户信息
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn delete_sys_user(headers: HeaderMap, State(state): State<Arc<AppState>>, Json(item): Json<DeleteUserReq>) -> impl IntoResponse {
    info!("delete sys_user params: {:?}", &item);
    let user_id = headers.get("user_id").unwrap().to_str().unwrap().parse::<i64>().unwrap();

    UserService::delete_sys_user(state, item, user_id).await
}

/*
 *更新用户信息
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn update_sys_user(State(state): State<Arc<AppState>>, Json(item): Json<UserReq>) -> impl IntoResponse {
    info!("update sys_user params: {:?}", &item);

    UserService::update_sys_user(state, item).await
}

/*
 *更新用户信息状态
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn update_sys_user_status(State(state): State<Arc<AppState>>, Json(item): Json<UpdateUserStatusReq>) -> impl IntoResponse {
    info!("update sys_user_status params: {:?}", &item);

    UserService::update_sys_user_status(state, item).await
}

/*
 *重置用户密码
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn reset_sys_user_password(State(state): State<Arc<AppState>>, Json(item): Json<ResetUserPwdReq>) -> impl IntoResponse {
    info!("update sys_user_password params: {:?}", &item);

    UserService::reset_sys_user_password(state, item).await
}

/*
 *用户修改自己的密码
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn update_sys_user_password(headers: HeaderMap, State(state): State<Arc<AppState>>, Json(item): Json<UpdateUserPwdReq>) -> impl IntoResponse {
    info!("update sys_user_password params: {:?}", &item);

    let user_id = headers.get("user_id").unwrap().to_str().unwrap().parse::<i64>().unwrap();

    UserService::update_sys_user_password(state, item, user_id).await
}

/*
 *查询用户信息详情
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn query_sys_user_detail(State(state): State<Arc<AppState>>, Json(item): Json<QueryUserDetailReq>) -> impl IntoResponse {
    info!("query sys_user_detail params: {:?}", &item);

    UserService::query_sys_user_detail(state, item).await
}

/*
 *查询用户信息列表
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn query_sys_user_list(State(state): State<Arc<AppState>>, Json(item): Json<QueryUserListReq>) -> impl IntoResponse {
    info!("query sys_user_list params: {:?}", &item);

    UserService::query_sys_user_list(state, item).await
}

/*
 *用户登录
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn login(headers: HeaderMap, State(state): State<Arc<AppState>>, Json(item): Json<UserLoginReq>) -> impl IntoResponse {
    info!("user login params: {:?}, {:?}", &item, state.batis);

    let user_agent = headers.get("User-Agent").unwrap().to_str().unwrap();

    UserService::login(state, item, user_agent).await
}

/*
 *查询用户角色
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn query_user_role(State(state): State<Arc<AppState>>, Json(item): Json<QueryUserRoleReq>) -> impl IntoResponse {
    info!("query user_role params: {:?}", item);

    UserService::query_user_role(state, item).await
}

// 更新用户角色
pub async fn update_user_role(State(state): State<Arc<AppState>>, Json(item): Json<UpdateUserRoleReq>) -> impl IntoResponse {
    info!("update_user_role params: {:?}", item);

    UserService::update_user_role(state, item).await
}

// 查询用户菜单
pub async fn query_user_menu(headers: HeaderMap, State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let user_id = headers.get("user_id").unwrap().to_str().unwrap().parse::<i64>().unwrap();
    info!("query user menu params user_id {:?}", user_id);

    UserService::query_user_menu(state, user_id).await
}
