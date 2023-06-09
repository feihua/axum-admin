use std::sync::Arc;
use axum::extract::{State};
use axum::{http::HeaderMap, Json};
use axum::response::IntoResponse;
use rbatis::rbdc::datetime::FastDateTime;
use rbatis::sql::{PageRequest};
use redis::Commands;
use crate::{AppState};
use crate::model::entity::{SysMenu, SysRole, SysRoleUser, SysUser};
use crate::utils::error::WhoUnfollowedError;
use crate::vo::user_vo::*;
use crate::utils::jwt_util::JWTToken;
use crate::utils::redis_util::init_redis;
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
            let id = user.id.unwrap().to_string();
            let username = user.real_name.unwrap();
            let password = user.password.unwrap();

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

            match JWTToken::new(&id, &username, btn_menu).create_token("123") {
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
            status_id: x.status_id.unwrap(),
            sort: x.sort.unwrap(),
            role_name: x.role_name.unwrap_or_default(),
            remark: x.remark.unwrap_or_default(),
            create_time: x.gmt_create.unwrap().0.to_string(),
            update_time: x.gmt_modified.unwrap().0.to_string(),
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

    let sys_result = SysRoleUser::delete_by_column(&mut rb, "user_id", user_id).await;

    if sys_result.is_err() {
        let resp = BaseResponse {
            msg: "更新用户角色异常".to_string(),
            code: 1,
            data: None,
        };
        return Json(resp);
    }

    let mut sys_role_user_list: Vec<SysRoleUser> = Vec::new();
    for role_id in role_ids {
        sys_role_user_list.push(SysRoleUser {
            id: None,
            gmt_create: Some(FastDateTime::now()),
            gmt_modified: Some(FastDateTime::now()),
            status_id: Some(1),
            sort: Some(1),
            role_id: Some(*role_id),
            user_id: Some(user_id),
        })
    }

    let result = SysRoleUser::insert_batch(&mut rb, &sys_role_user_list, len as u64).await;

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

    let sys_user = SysUser::select_by_column(&mut rb, "id", "1").await;

    let data = SysMenu::select_page(&mut rb, &PageRequest::new(1, 1000)).await;

    let mut sys_menu: Vec<MenuUserList> = Vec::new();
    let mut btn_menu: Vec<String> = Vec::new();
    let mut btn_menu_str: String = String::new();

    for x in data.unwrap().records {
        let y = x.clone();
        if y.menu_type != Some(3) {
            sys_menu.push(MenuUserList {
                id: y.id.unwrap(),
                parent_id: y.parent_id.unwrap(),
                name: y.menu_name.unwrap_or_default(),
                icon: y.menu_icon.unwrap_or_default(),
                api_url: y.api_url.as_ref().unwrap().to_string(),
                menu_type: y.menu_type.unwrap(),
                path: y.menu_url.unwrap_or_default(),
            });
        }

        btn_menu.push(x.api_url.unwrap_or_default());
        btn_menu_str.push_str(&x.menu_name.unwrap_or_default());
        btn_menu_str.push_str(&",")
    }

    let mut redis_con = init_redis().await;
    let _: () = redis_con.set("btn_menu_str", btn_menu_str).expect("sdfs");

    let resp = BaseResponse {
        msg: "successful".to_string(),
        code: 0,
        data: Some(QueryUserMenuData {
            sys_menu,
            btn_menu,
            avatar: "https://gw.alipayobjects.com/zos/antfincdn/XAosXuNZyF/BiazfanxmamNRoxxVxka.png".to_string(),
            name: sys_user.unwrap_or_default().get(0).unwrap().real_name.as_ref().expect("用户名不存在").to_string(),
        }),
    };
    Json(resp)
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
            let page_no = d.page_no;
            let page_size = d.page_size;

            let mut user_list: Vec<UserListData> = Vec::new();

            for x in d.records {
                user_list.push(UserListData {
                    id: x.id.unwrap(),
                    sort: x.sort.unwrap(),
                    status_id: x.status_id.unwrap(),
                    mobile: x.mobile.unwrap_or_default(),
                    real_name: x.real_name.unwrap_or_default(),
                    remark: x.remark.unwrap_or_default(),
                    create_time: x.gmt_create.unwrap().0.to_string(),
                    update_time: x.gmt_modified.unwrap().0.to_string(),
                })
            }

            UserListResp {
                msg: "successful".to_string(),
                code: 0,
                page_no,
                page_size,
                success: true,
                total,
                data: Some(user_list),
            }
        }
        Err(err) => {
            UserListResp {
                msg: err.to_string(),
                code: 1,
                page_no: 0,
                page_size: 0,
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
        gmt_create: Some(FastDateTime::now()),
        gmt_modified: Some(FastDateTime::now()),
        status_id: Some(1),
        sort: Some(1),
        user_no: Some(1),
        mobile: Some(item.mobile),
        real_name: Some(item.real_name),
        remark: Some(item.remark),
        password: Some("123456".to_string()),
    };

    let result = SysUser::insert(&mut rb, &sys_user).await;

    Json(handle_result(result))
}


pub async fn user_update(State(state): State<Arc<AppState>>, Json(item): Json<UserUpdateReq>) -> impl IntoResponse {
    log::info!("user_update params: {:?}", &item);

    let mut rb = &state.batis;
    let sys_user = SysUser {
        id: Some(item.id),
        gmt_create: None,
        gmt_modified: Some(FastDateTime::now()),
        status_id: Some(item.status_id),
        sort: Some(item.sort),
        user_no: None,
        mobile: Some(item.mobile),
        real_name: Some(item.real_name),
        remark: Some(item.remark),
        password: None,
    };

    let result = SysUser::update_by_column(&mut rb, &sys_user, "id").await;

    Json(handle_result(result))
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

    let user_result = SysUser::select_by_id(&mut rb, &item.id).await;

    match user_result {
        Ok(user) => {
            let mut sys_user = user.unwrap();
            sys_user.password = Some(item.re_pwd);
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