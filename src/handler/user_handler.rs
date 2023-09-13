use std::collections::HashSet;

use axum::{http::HeaderMap, Json};
use axum::response::IntoResponse;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, sql_query};
use diesel::associations::HasTable;
use diesel::sql_types::Bigint;
use log::{debug, error, info, warn};

use crate::{RB, schema};
use crate::model::menu::{StringColumn, SysMenu};
use crate::model::role::SysRole;
use crate::model::user::{SysUser, SysUserAdd, SysUserUpdate};
use crate::model::user_role::{SysUserRole, SysUserRoleAdd};
use crate::schema::sys_menu::{api_url, sort};
use crate::schema::sys_menu::dsl::sys_menu;
use crate::schema::sys_role::dsl::sys_role;
use crate::schema::sys_user::{id, mobile, status_id};
use crate::schema::sys_user::dsl::sys_user;
use crate::schema::sys_user_role::{role_id, user_id};
use crate::schema::sys_user_role::dsl::sys_user_role;
use crate::utils::error::WhoUnfollowedError;
use crate::utils::jwt_util::JWTToken;
use crate::vo::{err_result_msg, handle_result, ok_result, ok_result_data, ok_result_page};
use crate::vo::user_vo::*;

// 后台用户登录
pub async fn login(Json(item): Json<UserLoginReq>) -> impl IntoResponse {
    info!("user login params: {:?}", &item);
    match &mut RB.clone().get() {
        Ok(conn) => {
            let query = sys_user.filter(mobile.eq(&item.mobile));
            debug!("SQL: {}", diesel::debug_query::<diesel::mysql::Mysql, _>(&query).to_string());

            if let Ok(user) = query.first::<SysUser>(conn) {
                info!("select_by_mobile: {:?}", user);

                if user.password.ne(&item.password) {
                    return Json(err_result_msg("密码不正确".to_string()));
                }

                let btn_menu = query_btn_menu(user.id);

                if btn_menu.len() == 0 {
                    return Json(err_result_msg("用户没有分配角色或者菜单,不能登录".to_string()));
                }

                match JWTToken::new(user.id, &user.user_name, btn_menu).create_token("123") {
                    Ok(token) => {
                        Json(ok_result_data(token))
                    }
                    Err(err) => {
                        let er = match err {
                            WhoUnfollowedError::JwtTokenError(s) => { s }
                            _ => "no math error".to_string()
                        };

                        error!("err:{}", er.to_string());
                        Json(err_result_msg(er))
                    }
                }
            } else {
                error!("err:{}", "根据手机号查询用户异常".to_string());
                Json(err_result_msg("根据手机号查询用户异常".to_string()))
            }
        }
        Err(err) => {
            error!("err:{}", err.to_string());
            Json(err_result_msg(err.to_string()))
        }
    }
}

fn query_btn_menu(u_id: i64) -> Vec<String> {
    match &mut RB.clone().get() {
        Ok(conn) => {
            let user_role_sql = sql_query("SELECT * FROM sys_user_role where user_id = ? and role_id = 1");
            match user_role_sql.bind::<Bigint, _>(&u_id).get_result::<SysUserRole>(conn) {
                Ok(_) => {
                    let sys_menu_result = sys_menu.select(api_url).load::<String>(conn);
                    match sys_menu_result {
                        Ok(btn) => {
                            btn
                        }
                        Err(_) => {
                            Vec::new()
                        }
                    }
                }
                Err(_) => {
                    let result = sql_query("select u.api_url from sys_user_role t \
                    left join sys_role usr on t.role_id = usr.id \
                    left join sys_role_menu srm on usr.id = srm.role_id \
                    left join sys_menu u on srm.menu_id = u.id \
                    where t.user_id = ?")
                        .bind::<Bigint, _>(&u_id)
                        .load::<StringColumn>(conn);

                    match result {
                        Ok(btn_list) => {
                            let mut btn_list_data: Vec<String> = Vec::new();
                            for x in btn_list {
                                if x.api_url.len() != 0 {
                                    btn_list_data.push(x.api_url);
                                }
                            }
                            return btn_list_data;
                        }
                        Err(_) => {
                            Vec::new()
                        }
                    }
                }
            }
        }
        Err(err) => {
            error!("err:{}", err.to_string());
            Vec::new()
        }
    }
}

pub async fn query_user_role(Json(item): Json<QueryUserRoleReq>) -> Result<impl IntoResponse, impl IntoResponse> {
    info!("query_user_role params: {:?}", item);
    match &mut RB.clone().get() {
        Ok(conn) => {
            let mut user_role_ids: Vec<i64> = Vec::new();

            if let Ok(ids) = sys_user_role.filter(user_id.eq(&item.user_id)).select(role_id).load::<i64>(conn) {
                user_role_ids = ids
            }

            let sys_role_result = sys_role.load::<SysRole>(conn);
            let mut sys_role_list: Vec<UserRoleList> = Vec::new();

            if let Ok(role_list) = sys_role_result {
                for x in role_list {
                    sys_role_list.push(UserRoleList {
                        id: x.id,
                        status_id: x.status_id,
                        sort: x.sort,
                        role_name: x.role_name,
                        remark: x.remark,
                        create_time: x.create_time.to_string(),
                        update_time: x.update_time.to_string(),
                    });
                }
            }

            Ok(Json(ok_result_data(QueryUserRoleData {
                sys_role_list,
                user_role_ids,
            })))
        }
        Err(err) => {
            error!("err:{}", err.to_string());
            Err(Json(err_result_msg(err.to_string())))
        }
    }
}

pub async fn update_user_role(Json(item): Json<UpdateUserRoleReq>) -> impl IntoResponse {
    info!("update_user_role params: {:?}", item);
    let u_id = item.user_id;
    let role_ids = item.role_ids;

    if u_id == 1 {
        return Json(err_result_msg("不能修改超级管理员的角色".to_string()));
    }

    match &mut RB.clone().get() {
        Ok(conn) => {
            match diesel::delete(sys_user_role.filter(user_id.eq(u_id))).execute(conn) {
                Ok(_) => {
                    let mut sys_role_user_list: Vec<SysUserRoleAdd> = Vec::new();
                    for r_id in role_ids {
                        sys_role_user_list.push(SysUserRoleAdd {
                            status_id: 1,
                            sort: 1,
                            role_id: r_id,
                            user_id: u_id.clone(),
                        })
                    }
                    Json(handle_result(diesel::insert_into(sys_user_role::table()).values(&sys_role_user_list).execute(conn)))
                }
                Err(err) => {
                    error!("err:{}", err.to_string());
                    Json(err_result_msg(err.to_string()))
                }
            }
        }
        Err(err) => {
            error!("err:{}", err.to_string());
            Json(err_result_msg(err.to_string()))
        }
    }
}

pub async fn query_user_menu(headers: HeaderMap) -> Result<impl IntoResponse, impl IntoResponse> {
    let token = headers.get("Authorization").unwrap().to_str().unwrap();
    let split_vec = token.split_whitespace().collect::<Vec<_>>();
    if split_vec.len() != 2 || split_vec[0] != "Bearer" {
        return Err(Json(err_result_msg("the token format wrong".to_string())));
    }
    let token = split_vec[1];
    let jwt_token_e = JWTToken::verify("123", &token);
    let jwt_token = match jwt_token_e {
        Ok(data) => { data }
        Err(err) => {
            return match err {
                WhoUnfollowedError::JwtTokenError(er) => {
                    Err(Json(err_result_msg(er.to_string())))
                }
                _ => {
                    Err(Json(err_result_msg("other err".to_string())))
                }
            };
        }
    };

    info!("query user menu params {:?}",jwt_token);

    return match &mut RB.clone().get() {
        Ok(conn) => {
            match sql_query("select * from sys_user where id = ?").bind::<Bigint, _>(jwt_token.id).get_result::<SysUser>(conn) {
                Ok(user) => {
                    let user_role_sql = sql_query("SELECT * FROM sys_user_role where user_id = ? and role_id = 1");
                    let sys_menu_list: Vec<SysMenu>;
                    match user_role_sql.bind::<Bigint, _>(&user.id).get_result::<SysUserRole>(conn) {
                        Ok(_) => {
                            match sys_menu.load::<SysMenu>(conn) {
                                Ok(s_menus) => {
                                    sys_menu_list = s_menus;
                                }
                                Err(err) => {
                                    error!("err:{}", err.to_string());
                                    return Err(Json(err_result_msg(err.to_string())));
                                }
                            }
                        }
                        Err(_) => {
                            match sql_query("select u.* from sys_user_role t left join sys_role usr on t.role_id = usr.id left join sys_role_menu srm on usr.id = srm.role_id left join sys_menu u on srm.menu_id = u.id where t.user_id = ? order by u.id asc")
                                .bind::<Bigint, _>(&jwt_token.id)
                                .load::<SysMenu>(conn) {
                                Ok(s_menus) => {
                                    sys_menu_list = s_menus;
                                }
                                Err(err) => {
                                    error!("err:{}", err.to_string());
                                    return Err(Json(err_result_msg(err.to_string())));
                                }
                            }
                        }
                    }


                    let mut sys_user_menu_list: Vec<MenuUserList> = Vec::new();
                    let mut btn_menu: Vec<String> = Vec::new();
                    let mut sys_menu_ids: HashSet<i64> = HashSet::new();

                    for x in sys_menu_list {
                        if x.menu_type != 3 {
                            sys_menu_ids.insert(x.parent_id.clone());
                            sys_menu_ids.insert(x.id.clone());
                        }

                        if x.api_url.len() != 0 {
                            btn_menu.push(x.api_url);
                        }
                    }

                    let mut menu_ids = Vec::new();
                    for ids in sys_menu_ids {
                        menu_ids.push(ids)
                    }
                    match sys_menu.filter(schema::sys_menu::id.eq_any(menu_ids)).filter(schema::sys_menu::status_id.eq(1)).order(sort.asc()).distinct().load::<SysMenu>(conn) {
                        Ok(menu_list) => {
                            for x in menu_list {
                                sys_user_menu_list.push(MenuUserList {
                                    id: x.id,
                                    parent_id: x.parent_id,
                                    name: x.menu_name,
                                    icon: x.menu_icon.unwrap_or_default(),
                                    api_url: x.api_url.clone(),
                                    menu_type: x.menu_type,
                                    path: x.menu_url,
                                });

                                if x.api_url.len() != 0 {
                                    btn_menu.push(x.api_url);
                                }
                            }
                        }
                        Err(err) => {
                            error!("err:{}", err.to_string());
                            return Err(Json(err_result_msg(err.to_string())));
                        }
                    }

                    Ok(Json(ok_result_data(QueryUserMenuData {
                        sys_menu: sys_user_menu_list,
                        btn_menu,
                        avatar: "https://gw.alipayobjects.com/zos/antfincdn/XAosXuNZyF/BiazfanxmamNRoxxVxka.png".to_string(),
                        name: user.user_name,
                    })))
                }

                Err(err) => {
                    error!("err:{}", err.to_string());
                    return Err(Json(err_result_msg(err.to_string())));
                }
            }
        }
        Err(err) => {
            error!("err:{}", err.to_string());
            Err(Json(err_result_msg(err.to_string())))
        }
    };
}

// 查询用户列表
pub async fn user_list(Json(item): Json<UserListReq>) -> Result<impl IntoResponse, impl IntoResponse> {
    info!("query user_list params: {:?}", &item);
    let mut query = sys_user::table().into_boxed();
    if let Some(i) = &item.status_id {
        query = query.filter(status_id.eq(i));
    }
    if let Some(i) = &item.mobile {
        query = query.filter(mobile.eq(i));
    }

    debug!("SQL:{}", diesel::debug_query::<diesel::mysql::Mysql, _>(&query).to_string());

    match &mut RB.clone().get() {
        Ok(conn) => {
            let mut list_data: Vec<UserListData> = Vec::new();
            if let Ok(sys_user_list) = query.load::<SysUser>(conn) {
                for user in sys_user_list {
                    list_data.push(UserListData {
                        id: user.id,
                        sort: user.sort,
                        status_id: user.status_id,
                        mobile: user.mobile,
                        user_name: user.user_name,
                        remark: user.remark.unwrap_or_default(),
                        create_time: user.create_time.to_string(),
                        update_time: user.update_time.to_string(),
                    })
                }
            }
            Ok(Json(ok_result_page(list_data, 10)))
        }
        Err(err) => {
            error!("err:{}", err.to_string());
            Err(Json(err_result_msg(err.to_string())))
        }
    }
}

// 添加用户信息
pub async fn user_save(Json(user): Json<UserSaveReq>) -> impl IntoResponse {
    info!("user_save params: {:?}", &user);

    let s_user = SysUserAdd {
        status_id: user.status_id,
        sort: user.sort,
        mobile: user.mobile,
        user_name: user.user_name,
        remark: user.remark,
        password: "123456".to_string(),//默认密码为123456,暂时不加密
    };

    Json(SysUser::add_user(s_user))
}

// 更新用户信息
pub async fn user_update(Json(user): Json<UserUpdateReq>) -> impl IntoResponse {
    info!("user_update params: {:?}", &user);

    match &mut RB.clone().get() {
        Ok(conn) => {
            let user_sql = sql_query("SELECT * FROM sys_user where id = ? ");

            match user_sql.bind::<Bigint, _>(user.id).get_result::<SysUser>(conn) {
                Ok(s_user) => {
                    let s_user = SysUserUpdate {
                        id: user.id.clone(),
                        status_id: user.status_id,
                        sort: user.sort,
                        mobile: user.mobile,
                        user_name: user.user_name,
                        remark: user.remark,
                        password: s_user.password.clone(),
                    };

                    let query = diesel::update(sys_user.filter(id.eq(user.id.clone()))).set(s_user);
                    debug!("SQL:{}", diesel::debug_query::<diesel::mysql::Mysql, _>(&query).to_string());
                    Json(handle_result(query.execute(conn)))
                }
                Err(err) => {
                    error!("err:{}", err.to_string());
                    Json(err_result_msg(err.to_string()))
                }
            }
        }
        Err(err) => {
            error!("err:{}", err.to_string());
            Json(err_result_msg(err.to_string()))
        }
    }
}

// 删除用户信息
pub async fn user_delete(Json(item): Json<UserDeleteReq>) -> impl IntoResponse {
    info!("user_delete params: {:?}", &item);
    info!("user_delete params: {:?}", &item);

    match &mut RB.clone().get() {
        Ok(conn) => {
            let ids = item.ids.clone();
            //id为1的用户为系统预留用户,不能删除
            let mut delete_ids = vec![];
            for delete_id in ids {
                if delete_id == 1 {
                    warn!("err:{}", "不能删除超级管理员".to_string());
                    continue;
                }
                delete_ids.push(delete_id)
            }

            if delete_ids.len() == 0 {
                return Json(ok_result());
            }

            let query = diesel::delete(sys_user.filter(id.eq_any(delete_ids)));
            debug!("SQL: {}", diesel::debug_query::<diesel::mysql::Mysql, _>(&query).to_string());
            Json(handle_result(query.execute(conn)))
        }
        Err(err) => {
            error!("err:{}", err.to_string());
            Json(err_result_msg(err.to_string()))
        }
    }
}

// 更新用户密码
pub async fn update_user_password(Json(item): Json<UpdateUserPwdReq>) -> impl IntoResponse {
    info!("update_user_pwd params: {:?}", &item);

    match &mut RB.clone().get() {
        Ok(conn) => {
            let user_sql = sql_query("SELECT * FROM sys_user where id = ? ");
            match user_sql.bind::<Bigint, _>(item.id).get_result::<SysUser>(conn) {
                Ok(user) => {
                    if user.password != item.pwd {
                        error!("err:{}", "旧密码不正确".to_string());
                        return Json(err_result_msg("旧密码不正确".to_string()));
                    }
                    Json(handle_result(diesel::update(sys_user.filter(id.eq(item.id.clone()))).set(schema::sys_user::password.eq(&item.re_pwd)).execute(conn)))
                }
                Err(err) => {
                    error!("err:{}", err.to_string());
                    Json(err_result_msg(err.to_string()))
                }
            }
        }
        Err(err) => {
            error!("err:{}", err.to_string());
            Json(err_result_msg(err.to_string()))
        }
    }
}