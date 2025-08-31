use crate::common::error::AppError;
use crate::common::result::{err_result_msg, ok_result, ok_result_data, ok_result_page};
use crate::model::system::sys_dept_model::Dept;
use crate::model::system::sys_login_log_model::LoginLog;
use crate::model::system::sys_menu_model::Menu;
use crate::model::system::sys_role_model::Role;
use crate::model::system::sys_user_model::User;
use crate::model::system::sys_user_post_model::UserPost;
use crate::model::system::sys_user_role_model::{is_admin, UserRole};
use crate::utils::jwt_util::JwtToken;
use crate::utils::user_agent_util::UserAgentUtil;
use crate::vo::system::sys_dept_vo::DeptResp;
use crate::vo::system::sys_role_vo::RoleResp;
use crate::vo::system::sys_user_vo::*;
use crate::AppState;
use axum::extract::State;
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::Json;
use chrono::Local;
use rbatis::plugin::page::PageRequest;
use rbatis::rbatis_codegen::ops::AsProxy;
use rbatis::rbdc::DateTime;
use rbatis::RBatis;
use rbs::value;
use redis::Commands;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
/*
 *添加用户信息
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn add_sys_user(State(state): State<Arc<AppState>>, Json(item): Json<UserReq>) -> impl IntoResponse {
    log::info!("add sys_user params: {:?}", &item);
    let rb = &state.batis;

    if User::select_by_user_name(rb, &item.user_name).await?.is_some() {
        return Err(AppError::BusinessError("登录账号已存在"));
    }

    if User::select_by_mobile(rb, &item.mobile).await?.is_some() {
        return Err(AppError::BusinessError("手机号码已存在"));
    }

    if User::select_by_email(rb, &item.email).await?.is_some() {
        return Err(AppError::BusinessError("邮箱账号已存在"));
    }

    let post_ids = item.post_ids.clone();
    let id = User::insert(rb, &User::from(item)).await?.last_insert_id;

    let mut user_post_list: Vec<UserPost> = Vec::new();
    for post_id in post_ids {
        user_post_list.push(UserPost { user_id: id.i64(), post_id })
    }
    UserPost::insert_batch(rb, &user_post_list, user_post_list.len() as u64).await.map(|_| ok_result())?
}

/*
 *删除用户信息
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn delete_sys_user(headers: HeaderMap, State(state): State<Arc<AppState>>, Json(item): Json<DeleteUserReq>) -> impl IntoResponse {
    log::info!("delete sys_user params: {:?}", &item);
    let rb = &state.batis;
    let mut conn = state.redis.get_connection()?;

    let user_id = headers.get("user_id").unwrap().to_str().unwrap().parse::<i64>().unwrap();

    let ids = item.ids.clone();
    if ids.contains(&user_id) {
        return Err(AppError::BusinessError("当前用户不能删除"));
    }

    for id in ids.clone() {
        let key = format!("axum:admin:user:info:{}", id);
        let is_admin: bool = conn.hget(&key, "isAdmin").unwrap_or_default();

        if is_admin {
            return Err(AppError::BusinessError("不允许操作超级管理员用户"));
        }
    }

    UserRole::delete_by_map(rb, value! {"user_id": &ids}).await?;
    UserPost::delete_by_map(rb, value! {"user_id": &ids}).await?;
    User::delete_by_map(rb, value! {"id": &item.ids}).await.map(|_| ok_result())?
}

/*
 *更新用户信息
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn update_sys_user(State(state): State<Arc<AppState>>, Json(item): Json<UserReq>) -> impl IntoResponse {
    log::info!("update sys_user params: {:?}", &item);
    let rb = &state.batis;

    let mut conn = state.redis.get_connection()?;
    let id = item.id;
    let key = format!("axum:admin:user:info:{}", id.unwrap_or_default());
    let is_admin: bool = conn.hget(&key, "isAdmin").unwrap_or_default();

    if is_admin {
        return Err(AppError::BusinessError("不允许操作超级管理员用户"));
    }

    let user = match User::select_by_id(rb, id.unwrap_or_default()).await? {
        None => return Err(AppError::BusinessError("用户不存在")),
        Some(x) => x,
    };

    if let Some(x) = User::select_by_user_name(rb, &item.user_name).await? {
        if x.id != id {
            return Err(AppError::BusinessError("登录账号已存在"));
        }
    }

    if let Some(x) = User::select_by_mobile(rb, &item.mobile).await? {
        if x.id != id {
            return Err(AppError::BusinessError("手机号码已存在"));
        }
    }

    if let Some(x) = User::select_by_email(rb, &item.email).await? {
        if x.id != id {
            return Err(AppError::BusinessError("邮箱账号已存在"));
        }
    }

    let post_ids = item.post_ids.clone();
    let mut user_post_list: Vec<UserPost> = Vec::new();
    for post_id in post_ids {
        user_post_list.push(UserPost {
            user_id: user.id.unwrap_or_default(),
            post_id,
        })
    }

    UserPost::delete_by_map(rb, value! {"user_id": &item.id}).await?;
    UserPost::insert_batch(rb, &user_post_list, user_post_list.len() as u64).await?;

    let mut data = User::from(item);
    data.update_time = Some(DateTime::now());
    User::update_by_map(rb, &data, value! {"id": &id}).await.map(|_| ok_result())?
}

/*
 *更新用户信息状态
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn update_sys_user_status(State(state): State<Arc<AppState>>, Json(item): Json<UpdateUserStatusReq>) -> impl IntoResponse {
    log::info!("update sys_user_status params: {:?}", &item);
    let rb = &state.batis;

    let ids = item.ids.clone();
    let mut conn = state.redis.get_connection()?;

    for id in ids {
        let key = format!("axum:admin:user:info:{}", id);
        let is_admin: bool = conn.hget(&key, "isAdmin").unwrap_or_default();

        if is_admin {
            return Err(AppError::BusinessError("不允许操作超级管理员用户"));
        }
    }

    let update_sql = format!("update sys_user set status = ? where id in ({})", item.ids.iter().map(|_| "?").collect::<Vec<&str>>().join(", "));

    let mut param = vec![value!(item.status)];
    param.extend(item.ids.iter().map(|&id| value!(id)));
    rb.exec(&update_sql, param).await.map(|_| ok_result())?
}

/*
 *重置用户密码
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn reset_sys_user_password(State(state): State<Arc<AppState>>, Json(item): Json<ResetUserPwdReq>) -> impl IntoResponse {
    log::info!("update sys_user_password params: {:?}", &item);

    let rb = &state.batis;
    let mut conn = state.redis.get_connection()?;

    let key = format!("axum:admin:user:info:{}", item.id.clone());
    let is_admin: bool = conn.hget(&key, "isAdmin").unwrap_or_default();

    if is_admin {
        return Err(AppError::BusinessError("不允许操作超级管理员用户"));
    }

    let sys_user_result = User::select_by_id(rb, item.id).await?;

    match sys_user_result {
        None => Err(AppError::BusinessError("用户不存在")),
        Some(x) => {
            let mut user = x;
            user.password = item.password;
            User::update_by_map(rb, &user, value! {"id": &user.id}).await.map(|_| ok_result())?
        }
    }
}

/*
 *用户修改自己的密码
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn update_sys_user_password(headers: HeaderMap, State(state): State<Arc<AppState>>, Json(item): Json<UpdateUserPwdReq>) -> impl IntoResponse {
    log::info!("update sys_user_password params: {:?}", &item);

    let rb = &state.batis;

    let user_id = headers.get("user_id").unwrap().to_str().unwrap().parse::<i64>().unwrap();

    match User::select_by_id(rb, user_id).await? {
        None => Err(AppError::BusinessError("用户不存在")),
        Some(x) => {
            let mut user = x;
            if user.password != item.pwd {
                return Err(AppError::BusinessError("旧密码不正确"));
            }
            user.password = item.re_pwd;
            User::update_by_map(rb, &user, value! {"id": &user.id}).await.map(|_| ok_result())?
        }
    }
}

/*
 *查询用户信息详情
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn query_sys_user_detail(State(state): State<Arc<AppState>>, Json(item): Json<QueryUserDetailReq>) -> impl IntoResponse {
    log::info!("query sys_user_detail params: {:?}", &item);
    let rb = &state.batis;

    let mut x = match User::select_by_id(rb, item.id).await? {
        None => return Err(AppError::BusinessError("用户不存在")),
        Some(user) => {
            let a: UserResp = user.into();
            a
        }
    };

    let dept = match Dept::select_by_id(rb, &x.dept_id).await? {
        None => return Err(AppError::BusinessError("部门不存在")),

        Some(y) => {
            let a: DeptResp = y.into();
            Some(a)
        }
    };

    let post_ids = UserPost::select_by_map(rb, value! {"user_id": item.id}).await?.iter().map(|x| x.post_id).collect::<Vec<i64>>();

    x.dept_info = dept;
    x.post_ids = Some(post_ids);

    ok_result_data(x)
}

/*
 *查询用户信息列表
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn query_sys_user_list(State(state): State<Arc<AppState>>, Json(item): Json<QueryUserListReq>) -> impl IntoResponse {
    log::info!("query sys_user_list params: {:?}", &item);
    let rb = &state.batis;

    let page = &PageRequest::new(item.page_no, item.page_size);

    User::select_sys_user_list(rb, page, &item)
        .await
        .map(|x| ok_result_page(x.records.into_iter().map(|x| x.into()).collect::<Vec<UserResp>>(), x.total))?
}

/*
 *用户登录
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn login(headers: HeaderMap, State(state): State<Arc<AppState>>, Json(item): Json<UserLoginReq>) -> impl IntoResponse {
    log::info!("user login params: {:?}, {:?}", &item, state.batis);
    let rb = &state.batis;
    let mut conn = state.redis.get_connection()?;

    let user_agent = headers.get("User-Agent").unwrap().to_str().unwrap();
    log::info!("user agent: {:?}", user_agent);
    let agent = UserAgentUtil::new(user_agent);

    let user_result = User::select_by_mobile(rb, &item.mobile).await?;
    log::info!("query user by mobile: {:?}", user_result);

    match user_result {
        None => {
            add_login_log(rb, item.mobile, 0, "用户不存在", agent).await;
            Err(AppError::BusinessError("用户不存在"))
        }
        Some(user) => {
            let mut s_user = user.clone();
            let id = user.id.unwrap();
            let username = user.user_name;
            let password = user.password;

            if password.ne(&item.password) {
                add_login_log(rb, item.mobile, 0, "密码不正确", agent).await;
                return err_result_msg("密码不正确");
            }

            let (btn_menu, is_super) = query_btn_menu(&id, rb.clone()).await;

            if btn_menu.len() == 0 {
                add_login_log(rb, item.mobile, 0, "用户没有分配角色或者菜单,不能登录", agent).await;
                return Err(AppError::BusinessError("用户没有分配角色或者菜单,不能登录"));
            }

            let token = JwtToken::new(id, &username).create_token("123")?;

            let key = format!("axum:admin:user:info:{:?}", s_user.id.unwrap_or_default());
            // 存储用户权限信息
            conn.hset::<_, _, _, ()>(&key, "permissions", &btn_menu.join(","))?;
            // 存储用户名
            conn.hset::<_, _, _, ()>(&key, "user_name", &s_user.user_name)?;
            // 存储是否是超级管理员
            conn.hset::<_, _, _, ()>(&key, "isAdmin", is_super)?;
            // 存储token
            conn.hset::<_, _, _, ()>(&key, "token", &token)?;
            // 存储登录时间
            conn.hset::<_, _, _, ()>(&key, "last_login", Local::now().format("%Y-%m-%d %H:%M:%S").to_string())?;

            add_login_log(rb, item.mobile, 1, "登录成功", agent.clone()).await;
            s_user.login_os = agent.os;
            s_user.login_browser = agent.browser;
            s_user.login_date = Some(DateTime::now());
            User::update_by_map(rb, &s_user, value! {"id": &s_user.id}).await?;
            ok_result_data(token)
        }
    }
}

/*
 *添加登录日志
 *author：刘飞华
 *date：2025/01/02 17:01:13
 */
async fn add_login_log(rb: &RBatis, name: String, status: i8, msg: &str, agent: UserAgentUtil) {
    let sys_login_log = LoginLog {
        id: None,                             //访问ID
        login_name: name,                     //登录账号
        ipaddr: "todo".to_string(),           //登录IP地址
        login_location: "todo".to_string(),   //登录地点
        platform: agent.platform,             //平台信息
        browser: agent.browser,               //浏览器类型
        version: agent.version,               //浏览器版本
        os: agent.os,                         //操作系统
        arch: agent.arch,                     //体系结构信息
        engine: agent.engine,                 //渲染引擎信息
        engine_details: agent.engine_details, //渲染引擎详细信息
        extra: agent.extra,                   //其他信息（可选）
        status,                               //登录状态(0:失败,1:成功)
        msg: msg.to_string(),                 //提示消息
        login_time: None,                     //访问时间
    };

    match LoginLog::insert(rb, &sys_login_log).await {
        Ok(_u) => log::info!("add_login_log success: {:?}", sys_login_log),
        Err(err) => log::error!("add_login_log error params: {:?}, error message: {:?}", sys_login_log, err),
    }
}

/*
 *查询按钮权限
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
async fn query_btn_menu(id: &i64, rb: RBatis) -> (Vec<String>, bool) {
    let count = is_admin(&rb, id).await.unwrap_or_default();
    let mut btn_menu: Vec<String> = Vec::new();
    if count == 1 {
        let data = Menu::select_all(&rb).await;

        for x in data.unwrap_or_default() {
            if let Some(a) = x.api_url {
                if a != "" {
                    btn_menu.push(a);
                }
            }
        }
        log::info!("admin login: {:?}", id);
        (btn_menu, true)
    } else {
        let btn_menu_map: Vec<HashMap<String, String>> = rb.query_decode("select distinct u.api_url from sys_user_role t left join sys_role usr on t.role_id = usr.id left join sys_role_menu srm on usr.id = srm.role_id left join sys_menu u on srm.menu_id = u.id where t.user_id = ?", vec![value!(id)]).await.unwrap();
        for x in btn_menu_map {
            if let Some(a) = x.get("api_url") {
                if a.to_string() != "" {
                    btn_menu.push(a.to_string());
                }
            }
        }
        log::info!("ordinary login: {:?}", id);
        (btn_menu, false)
    }
}

/*
 *查询用户角色
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn query_user_role(State(state): State<Arc<AppState>>, Json(item): Json<QueryUserRoleReq>) -> impl IntoResponse {
    log::info!("query user_role params: {:?}", item);
    let rb = &state.batis;

    let mut user_role_ids: Vec<i64> = Vec::new();
    let sys_role_list = Role::select_all(rb).await.map(|x| x.into_iter().map(|x| x.into()).collect::<Vec<RoleResp>>())?;

    if item.user_id != 1 {
        let vec1 = UserRole::select_by_map(rb, value! {"user_id": item.user_id}).await?;
        for x in vec1 {
            user_role_ids.push(x.role_id);
        }
    }
    ok_result_data(QueryUserRoleResp { sys_role_list, user_role_ids })
}

// 更新用户角色
pub async fn update_user_role(State(state): State<Arc<AppState>>, Json(item): Json<UpdateUserRoleReq>) -> impl IntoResponse {
    log::info!("update_user_role params: {:?}", item);
    let rb = &state.batis;
    let mut conn = state.redis.get_connection()?;

    let user_id = item.user_id;
    let role_ids = &item.role_ids;
    let len = item.role_ids.len();

    let key = format!("axum:admin:user:info:{}", user_id);
    let is_admin: bool = conn.hget(&key, "isAdmin").unwrap_or_default();

    if is_admin {
        return Err(AppError::BusinessError("不允许操作超级管理员用户"));
    }

    UserRole::delete_by_map(rb, value! {"user_id": user_id}).await?;

    let mut list: Vec<UserRole> = Vec::new();
    for role_id in role_ids {
        let r_id = role_id.clone();
        list.push(UserRole {
            id: None,
            create_time: Some(DateTime::now()),
            role_id: r_id,
            user_id: user_id.clone(),
        })
    }

    UserRole::insert_batch(rb, &list, len as u64).await.map(|_| ok_result())?
}

// 查询用户菜单
pub async fn query_user_menu(headers: HeaderMap, State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let user_id = headers.get("user_id").unwrap().to_str().unwrap().parse::<i64>().unwrap();
    log::info!("query user menu params user_id {:?}", user_id);

    let rb = &state.batis;
    let mut conn = state.redis.get_connection()?;

    //根据id查询用户
    match User::select_by_id(rb, user_id).await? {
        None => Err(AppError::BusinessError("用户不存在")),
        Some(user) => {
            let key = format!("axum:admin:user:info:{}", user_id);
            let is_admin: bool = conn.hget(&key, "isAdmin").unwrap_or_default();

            let sys_menu_list: Vec<Menu>;

            if is_admin {
                log::info!("The current user is a super administrator");
                sys_menu_list = Menu::select_all(rb).await?;
            } else {
                log::info!("The current user is not a super administrator");
                let sql_str = "select u.* from sys_user_role t left join sys_role usr on t.role_id = usr.id left join sys_role_menu srm on usr.id = srm.role_id left join sys_menu u on srm.menu_id = u.id where t.user_id = ?";
                sys_menu_list = rb.query_decode(sql_str, vec![value!(user.id)]).await?;
            }

            let mut sys_menu: Vec<MenuList> = Vec::new();
            let mut btn_menu: Vec<String> = Vec::new();
            let mut sys_menu_ids: HashSet<i64> = HashSet::new();

            for x in sys_menu_list {
                if x.visible == 0 {
                    continue;
                }
                if x.menu_type != 3 {
                    sys_menu_ids.insert(x.id.unwrap_or_default().clone());
                    sys_menu_ids.insert(x.parent_id.unwrap_or_default().clone());
                }

                if x.api_url.clone().unwrap_or_default().len() > 0 {
                    btn_menu.push(x.api_url.unwrap_or_default());
                }
            }

            let mut menu_ids = Vec::new();
            for id in sys_menu_ids {
                menu_ids.push(id)
            }
            let vec1 = Menu::select_by_ids(rb, &menu_ids).await?;
            for menu in vec1 {
                sys_menu.push(MenuList {
                    id: menu.id,
                    parent_id: menu.parent_id,
                    name: menu.menu_name,
                    icon: menu.menu_icon.unwrap_or_default(),
                    api_url: menu.api_url.as_ref().map_or_else(|| "".to_string(), |url| url.to_string()),
                    menu_type: menu.menu_type,
                    path: menu.menu_url.unwrap_or_default(),
                });
            }

            let resp = QueryUserMenuResp {
                sys_menu,
                btn_menu,
                avatar: user.avatar,
                name: user.user_name,
            };

            ok_result_data(resp)
        }
    }
}
