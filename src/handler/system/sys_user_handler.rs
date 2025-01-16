use crate::common::result::BaseResponse;
use crate::model::system::sys_dept_model::Dept;
use crate::model::system::sys_login_log_model::LoginLog;
use crate::model::system::sys_menu_model::Menu;
use crate::model::system::sys_role_model::Role;
use crate::model::system::sys_user_model::User;
use crate::model::system::sys_user_post_model::UserPost;
use crate::model::system::sys_user_role_model::UserRole;
use crate::utils::error::WhoUnfollowedError;
use crate::utils::jwt_util::JWTToken;
use crate::utils::time_util::time_to_string;
use crate::utils::user_agent_util::UserAgentUtil;
use crate::vo::system::sys_dept_vo::QueryDeptDetailResp;
use crate::vo::system::sys_user_vo::*;
use crate::AppState;
use axum::extract::State;
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::Json;
use rbatis::plugin::page::PageRequest;
use rbatis::rbatis_codegen::ops::AsProxy;
use rbatis::rbdc::datetime::DateTime;
use rbatis::RBatis;
use rbs::to_value;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
/*
 *添加用户信息
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn add_sys_user(
    State(state): State<Arc<AppState>>,
    Json(item): Json<AddUserReq>,
) -> impl IntoResponse {
    log::info!("add sys_user params: {:?}", &item);
    let rb = &state.batis;

    let user_name_result = User::select_by_user_name(rb, &item.user_name).await;
    match user_name_result {
        Ok(r) => {
            if r.is_some() {
                return BaseResponse::<String>::err_result_msg("登录账号已存在".to_string());
            }
        }
        Err(err) => return BaseResponse::<String>::err_result_msg(err.to_string()),
    }

    let mobile_result = User::select_by_mobile(rb, &item.mobile).await;
    match mobile_result {
        Ok(r) => {
            if r.is_some() {
                return BaseResponse::<String>::err_result_msg("手机号码已存在".to_string());
            }
        }
        Err(err) => return BaseResponse::<String>::err_result_msg(err.to_string()),
    }

    let email_result = User::select_by_email(rb, &item.email).await;
    match email_result {
        Ok(r) => {
            if r.is_some() {
                return BaseResponse::<String>::err_result_msg("邮箱账号已存在".to_string());
            }
        }
        Err(err) => return BaseResponse::<String>::err_result_msg(err.to_string()),
    }

    let avatar = item.avatar.unwrap_or(
        "https://gw.alipayobjects.com/zos/antfincdn/XAosXuNZyF/BiazfanxmamNRoxxVxka.png"
            .to_string(),
    );
    let sys_user = User {
        id: None,                                  //主键
        mobile: item.mobile,                       //手机
        user_name: item.user_name,                 //用户账号
        nick_name: item.nick_name,                 //用户昵称
        user_type: Some("01".to_string()),         //用户类型（00系统用户）
        email: item.email,                         //用户邮箱
        avatar,                                    //头像路径
        password: item.password,                   //密码
        status: item.status,                       //状态(1:正常，0:禁用)
        dept_id: item.dept_id,                     //部门ID
        login_ip: "".to_string(),                  //最后登录IP
        login_date: None,                          //最后登录时间
        login_browser: "".to_string(),             //浏览器类型
        login_os: "".to_string(),                  //操作系统
        pwd_update_date: Some(Default::default()), //密码最后更新时间
        remark: item.remark,                       //备注
        del_flag: 1,                               //删除标志（0代表删除 1代表存在）
        create_time: None,                         //创建时间
        update_time: None,                         //修改时间
    };

    let result = User::insert(rb, &sys_user).await;

    match result {
        Ok(u) => {
            let mut user_post_list: Vec<UserPost> = Vec::new();
            for post_id in item.post_ids {
                user_post_list.push(UserPost {
                    user_id: u.last_insert_id.i64(),
                    post_id,
                })
            }
            match UserPost::insert_batch(rb, &user_post_list, user_post_list.len() as u64).await {
                Ok(_u) => BaseResponse::<String>::ok_result(),
                Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
            }
        }
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

/*
 *删除用户信息
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn delete_sys_user(
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
    Json(item): Json<DeleteUserReq>,
) -> impl IntoResponse {
    log::info!("delete sys_user params: {:?}", &item);
    let rb = &state.batis;

    let user_id = headers
        .get("user_id")
        .unwrap()
        .to_str()
        .unwrap()
        .parse::<i64>()
        .unwrap();

    let ids = item.ids.clone();
    if ids.contains(&user_id) {
        return BaseResponse::<String>::err_result_msg("当前用户不能删除".to_string());
    }
    if ids.contains(&1) {
        return BaseResponse::<String>::err_result_msg("不允许操作超级管理员用户".to_string());
    }

    let delete_user_role_result = UserRole::delete_in_column(rb, "user_id", &ids).await;
    match delete_user_role_result {
        Err(err) => return BaseResponse::<String>::err_result_msg(err.to_string()),
        _ => {}
    }

    let delete_user_post_result = UserPost::delete_in_column(rb, "user_id", &ids).await;
    match delete_user_post_result {
        Err(err) => return BaseResponse::<String>::err_result_msg(err.to_string()),
        _ => {}
    }

    let result = User::delete_in_column(rb, "id", &item.ids).await;

    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

/*
 *更新用户信息
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn update_sys_user(
    State(state): State<Arc<AppState>>,
    Json(item): Json<UpdateUserReq>,
) -> impl IntoResponse {
    log::info!("update sys_user params: {:?}", &item);
    let rb = &state.batis;

    let id = item.id.clone();
    if id == 1 {
        return BaseResponse::<String>::err_result_msg("不允许操作超级管理员用户".to_string());
    }

    let sys_user_result = User::select_by_id(rb, item.id).await;
    let u = match sys_user_result {
        Ok(user) => {
            if user.is_none() {
                return BaseResponse::<String>::err_result_msg("用户不存在".to_string());
            }
            user.unwrap()
        }
        Err(err) => return BaseResponse::<String>::err_result_msg(err.to_string()),
    };

    let user_name_result = User::select_by_user_name(rb, &item.user_name).await;
    match user_name_result {
        Ok(r) => {
            if r.is_some() && r.unwrap().id.unwrap_or_default() != item.id {
                return BaseResponse::<String>::err_result_msg("登录账号已存在".to_string());
            }
        }
        Err(err) => return BaseResponse::<String>::err_result_msg(err.to_string()),
    }

    let mobile_result = User::select_by_mobile(rb, &item.mobile).await;
    match mobile_result {
        Ok(r) => {
            if r.is_some() && r.unwrap().id.unwrap_or_default() != item.id {
                return BaseResponse::<String>::err_result_msg("手机号码已存在".to_string());
            }
        }
        Err(err) => return BaseResponse::<String>::err_result_msg(err.to_string()),
    }

    let email_result = User::select_by_email(rb, &item.email).await;
    match email_result {
        Ok(r) => {
            if r.is_some() && r.unwrap().id.unwrap_or_default() != item.id {
                return BaseResponse::<String>::err_result_msg("邮箱账号已存在".to_string());
            }
        }
        Err(err) => return BaseResponse::<String>::err_result_msg(err.to_string()),
    }

    let avatar = item.avatar.unwrap_or(
        "https://gw.alipayobjects.com/zos/antfincdn/XAosXuNZyF/BiazfanxmamNRoxxVxka.png"
            .to_string(),
    );
    let sys_user = User {
        id: Some(item.id),                          //主键
        mobile: item.mobile,                        //手机
        user_name: item.user_name,                  //用户账号
        nick_name: item.nick_name,                  //用户昵称
        user_type: None,                            //用户类型（00系统用户）
        email: item.email,                          //用户邮箱
        avatar,                                     //头像路径
        password: u.password,                       //密码
        status: item.status,                        //状态(1:正常，0:禁用)
        dept_id: item.dept_id,                      //部门ID
        login_ip: "item.login_ip".to_string(),      //最后登录IP
        login_date: Some(Default::default()),       //最后登录时间
        login_browser: "item.login_ip".to_string(), //浏览器类型
        login_os: "item.login_ip".to_string(),      //操作系统
        pwd_update_date: Some(Default::default()),  //密码最后更新时间
        remark: item.remark,                        //备注
        del_flag: u.del_flag,                       //删除标志（0代表删除 1代表存在）
        create_time: None,                          //创建时间
        update_time: None,                          //修改时间
    };

    let result = User::update_by_column(rb, &sys_user, "id").await;

    match result {
        Ok(_u) => {
            let _ = UserPost::delete_by_column(rb, "user_id", &item.id).await;
            let mut user_post_list: Vec<UserPost> = Vec::new();
            for post_id in item.post_ids {
                user_post_list.push(UserPost {
                    user_id: sys_user.id.unwrap_or_default(),
                    post_id,
                })
            }
            match UserPost::insert_batch(rb, &user_post_list, user_post_list.len() as u64).await {
                Ok(_u) => BaseResponse::<String>::ok_result(),
                Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
            }
        }
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

/*
 *更新用户信息状态
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn update_sys_user_status(
    State(state): State<Arc<AppState>>,
    Json(item): Json<UpdateUserStatusReq>,
) -> impl IntoResponse {
    log::info!("update sys_user_status params: {:?}", &item);
    let rb = &state.batis;

    let ids = item.ids.clone();
    if ids.contains(&1) {
        return BaseResponse::<String>::err_result_msg("不允许操作超级管理员用户".to_string());
    }

    let update_sql = format!(
        "update sys_user set status = ? where id in ({})",
        item.ids
            .iter()
            .map(|_| "?")
            .collect::<Vec<&str>>()
            .join(", ")
    );

    let mut param = vec![to_value!(item.status)];
    param.extend(item.ids.iter().map(|&id| to_value!(id)));
    let result = rb.exec(&update_sql, param).await;

    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

/*
 *重置用户密码
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn reset_sys_user_password(
    State(state): State<Arc<AppState>>,
    Json(item): Json<ResetUserPwdReq>,
) -> impl IntoResponse {
    log::info!("update sys_user_password params: {:?}", &item);

    let rb = &state.batis;

    let id = item.id.clone();
    if id == 1 {
        return BaseResponse::<String>::err_result_msg("不允许操作超级管理员用户".to_string());
    }

    let sys_user_result = User::select_by_id(rb, item.id).await;

    match sys_user_result {
        Ok(opt_user) => {
            if opt_user.is_none() {
                return BaseResponse::<String>::err_result_msg("用户不存在".to_string());
            }
            let mut user = opt_user.unwrap();
            user.password = item.password;
            let result = User::update_by_column(rb, &user, "id").await;
            match result {
                Ok(_u) => BaseResponse::<String>::ok_result(),
                Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
            }
        }
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

/*
 *用户修改自己的密码
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn update_sys_user_password(
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
    Json(item): Json<UpdateUserPwdReq>,
) -> impl IntoResponse {
    log::info!("update sys_user_password params: {:?}", &item);

    let rb = &state.batis;

    let user_id = headers
        .get("user_id")
        .unwrap()
        .to_str()
        .unwrap()
        .parse::<i64>()
        .unwrap();

    let sys_user_result = User::select_by_id(rb, user_id).await;

    match sys_user_result {
        Ok(opt_user) => {
            if opt_user.is_none() {
                return BaseResponse::<String>::err_result_msg("用户不存在".to_string());
            }
            let mut user = opt_user.unwrap();
            if user.password != item.pwd {
                return BaseResponse::<String>::err_result_msg("旧密码不正确".to_string());
            }
            user.password = item.re_pwd;
            let result = User::update_by_column(rb, &user, "id").await;

            match result {
                Ok(_u) => BaseResponse::<String>::ok_result(),
                Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
            }
        }
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

/*
 *查询用户信息详情
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn query_sys_user_detail(
    State(state): State<Arc<AppState>>,
    Json(item): Json<QueryUserDetailReq>,
) -> impl IntoResponse {
    log::info!("query sys_user_detail params: {:?}", &item);
    let rb = &state.batis;

    let result = User::select_by_id(rb, item.id).await;

    match result {
        Ok(d) => {
            if d.is_none() {
                return BaseResponse::<QueryUserDetailResp>::err_result_data(
                    QueryUserDetailResp::new(),
                    "用户不存在".to_string(),
                );
            }
            let x = d.unwrap();

            let dept_result = Dept::select_by_id(rb, &x.dept_id).await;
            let dept = match dept_result {
                Ok(opt_dept) => {
                    if opt_dept.is_none() {
                        return BaseResponse::<QueryUserDetailResp>::err_result_data(
                            QueryUserDetailResp::new(),
                            "查询用户详细信息失败,部门不存在".to_string(),
                        );
                    }
                    let x = opt_dept.unwrap();
                    QueryDeptDetailResp {
                        id: x.id.unwrap_or_default(),               //部门id
                        parent_id: x.parent_id,                     //父部门id
                        ancestors: x.ancestors,                     //祖级列表
                        dept_name: x.dept_name,                     //部门名称
                        sort: x.sort,                               //显示顺序
                        leader: x.leader,                           //负责人
                        phone: x.phone,                             //联系电话
                        email: x.email,                             //邮箱
                        status: x.status,                           //部状态（0：停用，1:正常）
                        del_flag: x.del_flag.unwrap_or_default(), //删除标志（0代表删除 1代表存在）
                        create_time: time_to_string(x.create_time), //创建时间
                        update_time: time_to_string(x.update_time), //修改时间
                    }
                }
                Err(err) => {
                    return BaseResponse::<QueryUserDetailResp>::err_result_data(
                        QueryUserDetailResp::new(),
                        err.to_string(),
                    )
                }
            };

            let result = UserPost::select_by_column(rb, "user_id", item.id)
                .await
                .unwrap_or_default();
            let post_ids = result.iter().map(|x| x.post_id).collect::<Vec<i64>>();

            let sys_user = QueryUserDetailResp {
                id: x.id.unwrap_or_default(),                       //主键
                mobile: x.mobile,                                   //手机
                user_name: x.user_name,                             //姓名
                nick_name: x.nick_name,                             //用户昵称
                user_type: x.user_type.unwrap_or_default(),         //用户类型（00系统用户）
                email: x.email,                                     //用户邮箱
                avatar: x.avatar,                                   //头像路径
                status: x.status,                                   //状态(1:正常，0:禁用)
                dept_id: x.dept_id,                                 //部门ID
                login_ip: x.login_ip,                               //最后登录IP
                login_date: time_to_string(x.login_date),           //最后登录时间
                login_browser: x.login_browser,                     //浏览器类型
                login_os: x.login_os,                               //操作系统
                pwd_update_date: time_to_string(x.pwd_update_date), //密码最后更新时间
                remark: x.remark,                                   //备注
                del_flag: x.del_flag, //删除标志（0代表删除 1代表存在）
                create_time: time_to_string(x.create_time), //创建时间
                update_time: time_to_string(x.update_time), //修改时间
                dept_info: dept,
                post_ids,
            };

            BaseResponse::<QueryUserDetailResp>::ok_result_data(sys_user)
        }
        Err(err) => BaseResponse::<QueryUserDetailResp>::err_result_data(
            QueryUserDetailResp::new(),
            err.to_string(),
        ),
    }
}

/*
 *查询用户信息列表
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn query_sys_user_list(
    State(state): State<Arc<AppState>>,
    Json(item): Json<QueryUserListReq>,
) -> impl IntoResponse {
    log::info!("query sys_user_list params: {:?}", &item);
    let rb = &state.batis;

    let mobile = item.mobile.as_deref().unwrap_or_default();
    let user_name = item.user_name.as_deref().unwrap_or_default();
    let status = item.status.unwrap_or(2);
    let dept_id = item.dept_id.unwrap_or_default();

    let page = &PageRequest::new(item.page_no, item.page_size);
    let result = User::select_sys_user_list(rb, page, mobile, user_name, status, dept_id).await;

    match result {
        Ok(d) => {
            let total = d.total;
            let mut sys_user_list_data: Vec<UserListDataResp> = Vec::new();
            for x in d.records {
                sys_user_list_data.push(UserListDataResp {
                    id: x.id.unwrap_or_default(),                       //主键
                    mobile: x.mobile,                                   //手机
                    user_name: x.user_name,                             //姓名
                    nick_name: x.nick_name,                             //用户昵称
                    user_type: x.user_type.unwrap_or_default(),         //用户类型（00系统用户）
                    email: x.email,                                     //用户邮箱
                    avatar: x.avatar,                                   //头像路径
                    status: x.status,                                   //状态(1:正常，0:禁用)
                    dept_id: x.dept_id,                                 //部门ID
                    login_ip: x.login_ip,                               //最后登录IP
                    login_date: time_to_string(x.login_date),           //最后登录时间
                    login_browser: x.login_browser,                     //浏览器类型
                    login_os: x.login_os,                               //操作系统
                    pwd_update_date: time_to_string(x.pwd_update_date), //密码最后更新时间
                    remark: x.remark,                                   //备注
                    del_flag: x.del_flag, //删除标志（0代表删除 1代表存在）
                    create_time: time_to_string(x.create_time), //创建时间
                    update_time: time_to_string(x.update_time), //修改时间
                })
            }

            BaseResponse::ok_result_page(sys_user_list_data, total)
        }
        Err(err) => BaseResponse::err_result_page(UserListDataResp::new(), err.to_string()),
    }
}

/*
 *用户登录
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn login(
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
    Json(item): Json<UserLoginReq>,
) -> impl IntoResponse {
    log::info!("user login params: {:?}, {:?}", &item, state.batis);
    let rb = &state.batis;

    let user_agent = headers.get("User-Agent").unwrap().to_str().unwrap();
    log::info!("user agent: {:?}", user_agent);
    let agent = UserAgentUtil::new(user_agent);

    let user_result = User::select_by_mobile(rb, &item.mobile).await;
    log::info!("query user by mobile: {:?}", user_result);

    match user_result {
        Ok(u) => match u {
            None => {
                add_login_log(rb, item.mobile, 0, "用户不存在".to_string(), agent).await;
                BaseResponse::<String>::err_result_msg("用户不存在".to_string())
            }
            Some(user) => {
                let mut s_user = user.clone();
                let id = user.id.unwrap();
                let username = user.user_name;
                let password = user.password;

                if password.ne(&item.password) {
                    add_login_log(rb, item.mobile, 0, "密码不正确".to_string(), agent).await;
                    return BaseResponse::<String>::err_result_msg("密码不正确".to_string());
                }

                let btn_menu = query_btn_menu(&id, rb.clone()).await;

                if btn_menu.len() == 0 {
                    add_login_log(
                        rb,
                        item.mobile,
                        0,
                        "用户没有分配角色或者菜单,不能登录".to_string(),
                        agent,
                    )
                    .await;
                    return BaseResponse::<String>::err_result_msg(
                        "用户没有分配角色或者菜单,不能登录".to_string(),
                    );
                }

                match JWTToken::new(id, &username, btn_menu).create_token("123") {
                    Ok(token) => {
                        add_login_log(rb, item.mobile, 1, "登录成功".to_string(), agent.clone())
                            .await;
                        s_user.login_os = agent.os;
                        s_user.login_browser = agent.browser;
                        s_user.login_date = Some(DateTime::now());
                        let res = User::update_by_column(rb, &s_user, "id").await;
                        if res.is_err() {
                            return BaseResponse::<String>::err_result_msg(
                                "更新用户登录后的信息失败".to_string(),
                            );
                        }
                        BaseResponse::<String>::ok_result_data(token)
                    }
                    Err(err) => {
                        let er = match err {
                            WhoUnfollowedError::JwtTokenError(s) => s,
                            _ => "no math error".to_string(),
                        };
                        add_login_log(rb, item.mobile, 0, "生成token异常".to_string(), agent).await;
                        BaseResponse::<String>::err_result_msg(er)
                    }
                }
            }
        },

        Err(err) => {
            add_login_log(rb, item.mobile, 0, "查询用户异常".to_string(), agent).await;
            log::info!("select_by_column: {:?}", err);
            BaseResponse::<String>::err_result_msg("查询用户异常".to_string())
        }
    }
}

/*
 *添加登录日志
 *author：刘飞华
 *date：2025/01/02 17:01:13
 */
async fn add_login_log(rb: &RBatis, name: String, status: i8, msg: String, agent: UserAgentUtil) {
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
        msg,                                  //提示消息
        login_time: None,                     //访问时间
    };

    match LoginLog::insert(rb, &sys_login_log).await {
        Ok(_u) => log::info!("add_login_log success: {:?}", sys_login_log),
        Err(err) => log::error!(
            "add_login_log error params: {:?}, error message: {:?}",
            sys_login_log,
            err.to_string()
        ),
    }
}

/*
 *查询按钮权限
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
async fn query_btn_menu(id: &i64, rb: RBatis) -> Vec<String> {
    let user_role = UserRole::is_admin(&rb, id).await;
    let mut btn_menu: Vec<String> = Vec::new();
    if user_role.unwrap().len() == 1 {
        let data = Menu::select_all(&rb).await;

        for x in data.unwrap() {
            btn_menu.push(x.api_url.unwrap_or_default());
        }
        log::info!("admin login: {:?}", id);
        btn_menu
    } else {
        let btn_menu_map: Vec<HashMap<String, String>> = rb.query_decode("select distinct u.api_url from sys_user_role t left join sys_role usr on t.role_id = usr.id left join sys_role_menu srm on usr.id = srm.role_id left join sys_menu u on srm.menu_id = u.id where t.user_id = ?", vec![to_value!(id)]).await.unwrap();
        for x in btn_menu_map {
            btn_menu.push(x.get("api_url").unwrap().to_string());
        }
        log::info!("ordinary login: {:?}", id);
        btn_menu
    }
}

/*
 *查询用户角色
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn query_user_role(
    State(state): State<Arc<AppState>>,
    Json(item): Json<QueryUserRoleReq>,
) -> impl IntoResponse {
    log::info!("query user_role params: {:?}", item);
    let rb = &state.batis;

    let mut user_role_ids: Vec<i64> = Vec::new();
    let mut sys_role_list: Vec<RoleList> = Vec::new();

    let sys_role = Role::select_all(rb).await;

    for x in sys_role.unwrap_or_default() {
        if x.status == 1 {
            let role = RoleList {
                id: x.id.unwrap_or_default(),               //主键
                role_name: x.role_name,                     //名称
                role_key: x.role_key,                       //角色权限字符串
                data_scope: x.data_scope, //数据范围（1：全部数据权限 2：自定数据权限 3：本部门数据权限 4：本部门及以下数据权限）
                status: x.status,         //状态(1:正常，0:禁用)
                remark: x.remark,         //备注
                del_flag: x.del_flag.unwrap_or_default(), //删除标志（0代表删除 1代表存在）
                create_time: time_to_string(x.create_time), //创建时间
                update_time: time_to_string(x.update_time), //修改时间
            };

            sys_role_list.push(role);
            user_role_ids.push(x.id.unwrap());
        }
    }

    if item.user_id != 1 {
        let user_role = UserRole::select_by_column(rb, "user_id", item.user_id).await;

        for x in user_role.unwrap_or_default() {
            user_role_ids.push(x.role_id);
        }
    }
    BaseResponse::<QueryUserRoleResp>::ok_result_data(QueryUserRoleResp {
        sys_role_list,
        user_role_ids,
    })
}

// 更新用户角色
pub async fn update_user_role(
    State(state): State<Arc<AppState>>,
    Json(item): Json<UpdateUserRoleReq>,
) -> impl IntoResponse {
    log::info!("update_user_role params: {:?}", item);
    let rb = &state.batis;

    let user_id = item.user_id;
    let role_ids = &item.role_ids;
    let len = item.role_ids.len();

    if user_id == 1 {
        return BaseResponse::<String>::err_result_msg("不能修改超级管理员的角色".to_string());
    }

    let sys_result = UserRole::delete_by_column(rb, "user_id", user_id).await;

    if sys_result.is_err() {
        return BaseResponse::<String>::err_result_msg("更新用户角色异常".to_string());
    }

    let mut sys_role_user_list: Vec<UserRole> = Vec::new();
    for role_id in role_ids {
        let r_id = role_id.clone();
        sys_role_user_list.push(UserRole {
            id: None,
            create_time: Some(DateTime::now()),
            role_id: r_id,
            user_id: user_id.clone(),
        })
    }

    let result = UserRole::insert_batch(rb, &sys_role_user_list, len as u64).await;

    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

// 查询用户菜单
pub async fn query_user_menu(
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let user_id = headers
        .get("user_id")
        .unwrap()
        .to_str()
        .unwrap()
        .parse::<i64>()
        .unwrap();
    log::info!("query user menu params user_id {:?}", user_id);

    let rb = &state.batis;

    //根据id查询用户
    let result = User::select_by_id(rb, user_id).await;

    match result {
        Ok(sys_user) => {
            match sys_user {
                None => Json(BaseResponse {
                    msg: "用户不存在".to_string(),
                    code: 1,
                    data: None,
                }),
                Some(user) => {
                    //role_id为1是超级管理员--判断是不是超级管理员
                    let sql_str =
                        "select count(id) from sys_user_role where role_id = 1 and user_id = ?";
                    let count = rb
                        .query_decode::<i32>(sql_str, vec![to_value!(user.id)])
                        .await
                        .unwrap_or_default();

                    let sys_menu_list: Vec<Menu>;

                    if count > 0 {
                        sys_menu_list = Menu::select_all(rb).await.unwrap_or_default();
                    } else {
                        let sql_str = "select u.* from sys_user_role t left join sys_role usr on t.role_id = usr.id left join sys_role_menu srm on usr.id = srm.role_id left join sys_menu u on srm.menu_id = u.id where t.user_id = ?";
                        sys_menu_list = rb
                            .query_decode(sql_str, vec![to_value!(user.id)])
                            .await
                            .unwrap();
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
                            sys_menu_ids.insert(x.parent_id.clone());
                        }

                        if x.api_url.clone().unwrap_or_default().len() > 0 {
                            btn_menu.push(x.api_url.unwrap_or_default());
                        }
                    }

                    let mut menu_ids = Vec::new();
                    for id in sys_menu_ids {
                        menu_ids.push(id)
                    }
                    let menu_result = Menu::select_by_ids(rb, &menu_ids).await.unwrap();
                    for menu in menu_result {
                        sys_menu.push(MenuList {
                            id: menu.id.unwrap(),
                            parent_id: menu.parent_id,
                            name: menu.menu_name,
                            icon: menu.menu_icon.unwrap_or_default(),
                            api_url: menu.api_url.as_ref().unwrap().to_string(),
                            menu_type: menu.menu_type,
                            path: menu.menu_url.unwrap_or_default(),
                        });
                    }

                    let resp = BaseResponse {
                        msg: "successful".to_string(),
                        code: 0,
                        data: Some(QueryUserMenuResp {
                            sys_menu,
                            btn_menu,
                            avatar: "https://gw.alipayobjects.com/zos/antfincdn/XAosXuNZyF/BiazfanxmamNRoxxVxka.png".to_string(),
                            name: user.user_name,
                        }),
                    };
                    Json(resp)
                }
            }
        }
        // 查询用户数据库异常
        Err(err) => Json(BaseResponse {
            msg: err.to_string(),
            code: 1,
            data: None,
        }),
    }
}
