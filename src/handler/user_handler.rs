use std::collections::HashMap;
use std::sync::Arc;
use axum::extract::{State};
use axum::{http::HeaderMap, Json};
use axum::response::IntoResponse;
use rbatis::rbdc::datetime::DateTime;
use rbatis::sql::{PageRequest};
use rbs::to_value;
use crate::{AppState};
use crate::model::user::{SysUser};
use crate::model::menu::{SysMenu};
use crate::model::role::{SysRole};
use crate::model::user_role::{SysUserRole};
use crate::utils::error::WhoUnfollowedError;
use crate::vo::user_vo::*;
use crate::utils::jwt_util::JWTToken;
use crate::vo::{BaseResponse, handle_result};


pub async fn login(State(state): State<Arc<AppState>>, Json(item): Json<UserLoginReq>) -> impl IntoResponse {
    log::info!("user login params: {:?}, {:?}", &item, state.batis);
    let mut rb = &state.batis;

    let user_result = SysUser::select_by_column(&mut rb, "mobile", &item.mobile).await;
    log::info!("select_by_column: {:?}",user_result);

    match user_result {
        Ok(d) => {
            if d.len() == 0 {
                let resp = BaseResponse {
                    msg: "用户不存在".to_string(),
                    code: 1,
                    data: None,
                };
                return Json(resp);
            }

            let user = d.get(0).unwrap().clone();
            let id = user.id.unwrap();
            let username = user.user_name;
            let password = user.password;

            if password.ne(&item.password) {
                let resp = BaseResponse {
                    msg: "密码不正确".to_string(),
                    code: 1,
                    data: None,
                };
                return Json(resp);
            }

            let data = SysMenu::select_page(&mut rb, &PageRequest::new(1, 1000)).await;

            let mut btn_menu: Vec<String> = Vec::new();

            for x in data.unwrap().records {
                btn_menu.push(x.api_url.unwrap_or_default());
            }

            match JWTToken::new(id, &username, btn_menu).create_token("123") {
                Ok(token) => {
                    let resp = BaseResponse {
                        msg: "successful".to_string(),
                        code: 0,
                        data: Some(UserLoginData {
                            mobile: item.mobile.to_string(),
                            token,
                        }),
                    };

                    Json(resp)
                }
                Err(err) => {
                    let er = match err {
                        WhoUnfollowedError::JwtTokenError(s) => { s }
                        _ => "no math error".to_string()
                    };
                    let resp = BaseResponse {
                        msg: er,
                        code: 1,
                        data: None,
                    };

                    Json(resp)
                }
            }
        }

        Err(err) => {
            log::info!("select_by_column: {:?}",err);
            let resp = BaseResponse {
                msg: "查询用户异常".to_string(),
                code: 1,
                data: None,
            };
            return Json(resp);
        }
    }
}

pub async fn query_user_role(State(state): State<Arc<AppState>>, Json(item): Json<QueryUserRoleReq>) -> impl IntoResponse {
    log::info!("query_user_role params: {:?}", item);
    let mut rb = &state.batis;

    let sys_role = SysRole::select_page(&mut rb, &PageRequest::new(1, 1000)).await;

    let mut sys_role_list: Vec<UserRoleList> = Vec::new();
    let mut user_role_ids: Vec<i32> = Vec::new();

    for x in sys_role.unwrap().records {
        sys_role_list.push(UserRoleList {
            id: x.id.unwrap(),
            status_id: x.status_id,
            sort: x.sort,
            role_name: x.role_name,
            remark: x.remark.unwrap_or_default(),
            create_time: x.create_time.unwrap().0.to_string(),
            update_time: x.update_time.unwrap().0.to_string(),
        });

        user_role_ids.push(x.id.unwrap_or_default());
    }

    let resp = QueryUserRoleResp {
        msg: "successful".to_string(),
        code: 0,
        data: QueryUserRoleData {
            sys_role_list,
            user_role_ids,
        },
    };

    Json(resp)
}

pub async fn update_user_role(State(state): State<Arc<AppState>>, Json(item): Json<UpdateUserRoleReq>) -> impl IntoResponse {
    log::info!("update_user_role params: {:?}", item);
    let mut rb = &state.batis;

    let user_id = item.user_id;
    let role_ids = &item.role_ids;
    let len = item.role_ids.len();

    if user_id == 1 {
        let resp = BaseResponse {
            msg: "不能修改超级管理员的角色".to_string(),
            code: 1,
            data: None,
        };
        return Json(resp);
    }

    let sys_result = SysUserRole::delete_by_column(&mut rb, "user_id", user_id).await;

    if sys_result.is_err() {
        let resp = BaseResponse {
            msg: "更新用户角色异常".to_string(),
            code: 1,
            data: None,
        };
        return Json(resp);
    }

    let mut sys_role_user_list: Vec<SysUserRole> = Vec::new();
    for role_id in role_ids {
        let r_id = role_id.clone();
        sys_role_user_list.push(SysUserRole {
            id: None,
            create_time: Some(DateTime::now()),
            update_time: Some(DateTime::now()),
            status_id: 1,
            sort: 1,
            role_id: r_id,
            user_id: user_id.clone(),
        })
    }

    let result = SysUserRole::insert_batch(&mut rb, &sys_role_user_list, len as u64).await;

    Json(handle_result(result))
}

pub async fn query_user_menu(headers: HeaderMap, State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let token = headers.get("Authorization").unwrap().to_str().unwrap();
    let split_vec = token.split_whitespace().collect::<Vec<_>>();
    if split_vec.len() != 2 || split_vec[0] != "Bearer" {
        let resp = BaseResponse {
            msg: "the token format wrong".to_string(),
            code: 1,
            data: None,
        };
        return Json(resp);
    }
    let token = split_vec[1];
    let jwt_token_e = JWTToken::verify("123", &token);
    let jwt_token = match jwt_token_e {
        Ok(data) => { data }
        Err(err) => {
            let resp = BaseResponse {
                msg: err.to_string(),
                code: 1,
                data: None,
            };
            return Json(resp);
        }
    };

    log::info!("query user menu params {:?}",jwt_token);

    let mut rb = &state.batis;

    //根据id查询用户
    let result = SysUser::select_by_id(&mut rb, jwt_token.id).await;

    match result {
        Ok(sys_user) => {
            match sys_user {
                // 用户不存在的情况
                None => {
                    Json(BaseResponse {
                        msg: "用户不存在".to_string(),
                        code: 1,
                        data: None,
                    })
                }
                Some(user) => {
                    let user_role = SysUserRole::select_by_column(&mut rb, "user_id", user.id).await;
                    // 判断是不是超级管理员
                    let mut is_admin = false;

                    for x in user_role.unwrap() {
                        if x.role_id == 1 {
                            is_admin = true;
                            break;
                        }
                    }

                    let sys_menu_list: Vec<SysMenu>;

                    if is_admin {
                        sys_menu_list = SysMenu::select_all(&mut rb).await.unwrap_or_default();
                    } else {
                        sys_menu_list = rb.query_decode("select u.* from sys_user_role t left join sys_role usr on t.role_id = usr.id left join sys_role_menu srm on usr.id = srm.role_id left join sys_menu u on srm.menu_id = u.id where t.user_id = ? order by u.id asc", vec![to_value!(user.id)]).await.unwrap();
                    }

                    let mut sys_menu_map: HashMap<i32, MenuUserList> = HashMap::new();
                    let mut sys_menu: Vec<MenuUserList> = Vec::new();
                    let mut btn_menu: Vec<String> = Vec::new();
                    let mut sys_menu_parent_ids: Vec<i32> = Vec::new();

                    for x in sys_menu_list {
                        let y = x.clone();
                        if y.menu_type != 3 {
                            sys_menu_map.insert(y.id.unwrap(), MenuUserList {
                                id: y.id.unwrap(),
                                parent_id: y.parent_id,
                                name: y.menu_name,
                                icon: y.menu_icon.unwrap_or_default(),
                                api_url: y.api_url.as_ref().unwrap().to_string(),
                                menu_type: y.menu_type,
                                path: y.menu_url.unwrap_or_default(),
                            });
                            sys_menu_parent_ids.push(y.parent_id.clone())
                        }

                        btn_menu.push(x.api_url.unwrap_or_default());
                    }

                    for menu_id in sys_menu_parent_ids {
                        let s_menu_result = SysMenu::select_by_id(&mut rb, menu_id).await.unwrap();
                        match s_menu_result {
                            None => {}
                            Some(y) => {
                                sys_menu_map.insert(y.id.unwrap(), MenuUserList {
                                    id: y.id.unwrap(),
                                    parent_id: y.parent_id,
                                    name: y.menu_name,
                                    icon: y.menu_icon.unwrap_or_default(),
                                    api_url: y.api_url.as_ref().unwrap().to_string(),
                                    menu_type: y.menu_type,
                                    path: y.menu_url.unwrap_or_default(),
                                });
                            }
                        }
                    }

                    let mut sys_menu_ids: Vec<i32> = Vec::new();
                    for menu in &sys_menu_map {
                        sys_menu_ids.push(menu.0.abs())
                    }

                    sys_menu_ids.sort();

                    for id in sys_menu_ids {
                        let menu = sys_menu_map.get(&id).cloned().unwrap();
                        sys_menu.push(menu)
                    }

                    let resp = BaseResponse {
                        msg: "successful".to_string(),
                        code: 0,
                        data: Some(QueryUserMenuData {
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
        Err(err) => {
            Json(BaseResponse {
                msg: err.to_string(),
                code: 1,
                data: None,
            })
        }
    }
}


pub async fn user_list(State(state): State<Arc<AppState>>, Json(item): Json<UserListReq>) -> impl IntoResponse {
    log::info!("query user_list params: {:?}", &item);
    let mut rb = &state.batis;

    let mobile = item.mobile.as_deref().unwrap_or_default();
    let status_id = item.status_id.as_deref().unwrap_or_default();

    let page = &PageRequest::new(item.page_no, item.page_size);
    let result = SysUser::select_page_by_name(&mut rb, page, mobile, status_id).await;

    let resp = match result {
        Ok(d) => {
            let total = d.total;

            let mut user_list: Vec<UserListData> = Vec::new();

            for x in d.records {
                user_list.push(UserListData {
                    id: x.id.unwrap(),
                    sort: x.sort,
                    status_id: x.status_id,
                    mobile: x.mobile,
                    user_name: x.user_name,
                    remark: x.remark.unwrap_or_default(),
                    create_time: x.create_time.unwrap().0.to_string(),
                    update_time: x.update_time.unwrap().0.to_string(),
                })
            }

            UserListResp {
                msg: "successful".to_string(),
                code: 0,
                success: true,
                total,
                data: Some(user_list),
            }
        }
        Err(err) => {
            UserListResp {
                msg: err.to_string(),
                code: 1,
                success: true,
                total: 0,
                data: None,
            }
        }
    };

    Json(resp)
}


pub async fn user_save(State(state): State<Arc<AppState>>, Json(item): Json<UserSaveReq>) -> impl IntoResponse {
    log::info!("user_save params: {:?}", &item);

    let mut rb = &state.batis;
    let sys_user = SysUser {
        id: None,
        create_time: Some(DateTime::now()),
        update_time: Some(DateTime::now()),
        status_id: item.status_id,
        sort: item.sort,
        mobile: item.mobile,
        user_name: item.user_name,
        remark: item.remark,
        password: "123456".to_string(),//默认密码为123456,暂时不加密
    };

    let result = SysUser::insert(&mut rb, &sys_user).await;

    Json(handle_result(result))
}


pub async fn user_update(State(state): State<Arc<AppState>>, Json(item): Json<UserUpdateReq>) -> impl IntoResponse {
    log::info!("user_update params: {:?}", &item);

    let mut rb = &state.batis;
    let result = SysUser::select_by_id(&mut rb, item.id.clone()).await.unwrap();

    match result {
        None => {
            Json(BaseResponse {
                msg: "用户不存在".to_string(),
                code: 1,
                data: Some("None".to_string()),
            })
        }
        Some(s_user) => {
            let sys_user = SysUser {
                id: Some(item.id),
                create_time: s_user.create_time,
                update_time: Some(DateTime::now()),
                status_id: item.status_id,
                sort: item.sort,
                mobile: item.mobile,
                user_name: item.user_name,
                remark: item.remark,
                password: s_user.password,
            };

            let result = SysUser::update_by_column(&mut rb, &sys_user, "id").await;

            Json(handle_result(result))
        }
    }
}


pub async fn user_delete(State(state): State<Arc<AppState>>, Json(item): Json<UserDeleteReq>) -> impl IntoResponse {
    log::info!("user_delete params: {:?}", &item);
    let mut rb = &state.batis;

    let result = SysUser::delete_in_column(&mut rb, "id", &item.ids).await;

    Json(handle_result(result))
}

pub async fn update_user_password(State(state): State<Arc<AppState>>, Json(item): Json<UpdateUserPwdReq>) -> impl IntoResponse {
    log::info!("update_user_pwd params: {:?}", &item);

    let mut rb = &state.batis;

    let user_result = SysUser::select_by_id(&mut rb, item.id).await;

    match user_result {
        Ok(user) => {
            let mut sys_user = user.unwrap();
            sys_user.password = item.re_pwd;
            let result = SysUser::update_by_column(&mut rb, &sys_user, "id").await;

            Json(handle_result(result))
        }
        Err(err) => {
            let resp = BaseResponse {
                msg: err.to_string(),
                code: 1,
                data: None,
            };
            Json(resp)
        }
    }
}