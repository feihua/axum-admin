use crate::common::result::BaseResponse;
use crate::model::system::sys_dept_model::Dept;
use crate::model::system::sys_menu_model::{select_count_menu_by_parent_id, Menu};
use crate::model::system::sys_post_model::Post;
use crate::model::system::sys_role_dept_model::RoleDept;
use crate::model::system::sys_role_menu_model::{query_menu_by_role, RoleMenu};
use crate::model::system::sys_role_model::Role;
use crate::model::system::sys_user_post_model::count_user_post_by_id;
use crate::model::system::sys_user_role_model::UserRole;
use crate::vo::system::sys_role_vo::*;
use crate::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use rbatis::plugin::page::PageRequest;
use rbatis::rbdc::datetime::DateTime;
use rbatis::rbdc::db::ExecResult;
use rbatis::rbdc::Error;
use rbs::to_value;
use std::sync::Arc;

/*
 *添加角色信息
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn add_sys_role(
    State(state): State<Arc<AppState>>,
    Json(item): Json<AddRoleReq>,
) -> impl IntoResponse {
    log::info!("add_sys_role params: {:?}", &item);
    let rb = &state.batis;

    let res = Role::select_by_role_name(rb, &item.role_name).await;
    match res {
        Ok(r) => {
            if r.is_some() {
                return BaseResponse::<String>::err_result_msg("角色名称已存在".to_string());
            }
        }
        Err(err) => return BaseResponse::<String>::err_result_msg(err.to_string()),
    }

    let res1 = Role::select_by_role_key(rb, &item.role_key).await;
    match res1 {
        Ok(r) => {
            if r.is_some() {
                return BaseResponse::<String>::err_result_msg("角色权限已存在".to_string());
            }
        }
        Err(err) => return BaseResponse::<String>::err_result_msg(err.to_string()),
    }

    let sys_role = Role {
        id: None,                                //主键
        role_name: item.role_name,               //名称
        role_key: item.role_key,                 //角色权限字符串
        data_scope: item.data_scope, //数据范围（1：全部数据权限 2：自定数据权限 3：本部门数据权限 4：本部门及以下数据权限）
        status: item.status,         //状态(1:正常，0:禁用)
        sort: item.sort,             //排序
        remark: item.remark.unwrap_or_default(), //备注
        del_flag: 1,                 //删除标志（0代表删除 1代表存在）
        create_time: None,           //创建时间
        update_time: None,           //修改时间
    };

    let result = Role::insert(rb, &sys_role).await;

    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

/*
 *删除角色信息
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn delete_sys_role(
    State(state): State<Arc<AppState>>,
    Json(item): Json<DeleteRoleReq>,
) -> impl IntoResponse {
    log::info!("delete_sys_role params: {:?}", &item);
    let rb = &state.batis;

    let ids = item.ids.clone();
    for id in ids {
        let post_by_id = Role::select_by_id(rb, &id).await;
        let p = match post_by_id {
            Ok(p) => {
                if p.is_none() {
                    return BaseResponse::<String>::err_result_msg(
                        "角色不存在,不能删除".to_string(),
                    );
                } else {
                    p.unwrap()
                }
            }
            Err(err) => return BaseResponse::<String>::err_result_msg(err.to_string()),
        };

        let res = count_user_post_by_id(rb, id).await;
        if res.unwrap_or_default() > 0 {
            let msg = format!("{}已分配,不能删除", p.role_name);
            return BaseResponse::<String>::err_result_msg(msg);
        }
    }

    let result = RoleMenu::delete_in_column(rb, "role_id", &item.ids).await;

    match result {
        Err(err) => return BaseResponse::<String>::err_result_msg(err.to_string()),
        _ => {}
    }
    let result1 = RoleDept::delete_in_column(rb, "role_id", &item.ids).await;

    match result1 {
        Err(err) => return BaseResponse::<String>::err_result_msg(err.to_string()),
        _ => {}
    }

    let result2 = Role::delete_in_column(rb, "id", &item.ids).await;

    match result2 {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

/*
 *更新角色信息
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn update_sys_role(
    State(state): State<Arc<AppState>>,
    Json(item): Json<UpdateRoleReq>,
) -> impl IntoResponse {
    log::info!("update_sys_role params: {:?}", &item);
    let rb = &state.batis;

    let res = Role::select_by_role_name(rb, &item.role_name).await;
    match res {
        Ok(r) => {
            if r.is_some() && r.unwrap().id.unwrap_or_default() != item.id {
                return BaseResponse::<String>::err_result_msg("角色名称已存在".to_string());
            }
        }
        Err(err) => return BaseResponse::<String>::err_result_msg(err.to_string()),
    }

    let res1 = Role::select_by_role_key(rb, &item.role_key).await;
    match res1 {
        Ok(r) => {
            if r.is_some() && r.unwrap().id.unwrap_or_default() != item.id {
                return BaseResponse::<String>::err_result_msg("角色权限已存在".to_string());
            }
        }
        Err(err) => return BaseResponse::<String>::err_result_msg(err.to_string()),
    }

    let sys_role = Role {
        id: Some(item.id),                       //主键
        role_name: item.role_name,               //名称
        role_key: item.role_key,                 //角色权限字符串
        data_scope: item.data_scope, //数据范围（1：全部数据权限 2：自定数据权限 3：本部门数据权限 4：本部门及以下数据权限）
        status: item.status,         //状态(1:正常，0:禁用)
        sort: item.sort,             //排序
        remark: item.remark.unwrap_or_default(), //备注
        del_flag: item.del_flag,     //删除标志（0代表删除 1代表存在）
        create_time: None,           //创建时间
        update_time: None,           //修改时间
    };

    let result = Role::update_by_column(rb, &sys_role, "id").await;

    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

/*
 *更新角色信息状态
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn update_sys_role_status(
    State(state): State<Arc<AppState>>,
    Json(item): Json<UpdateRoleStatusReq>,
) -> impl IntoResponse {
    log::info!("update_sys_role_status params: {:?}", &item);
    let rb = &state.batis;

    let update_sql = format!(
        "update sys_role set status = ? where id in ({})",
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
 *查询角色信息详情
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn query_sys_role_detail(
    State(state): State<Arc<AppState>>,
    Json(item): Json<QueryRoleDetailReq>,
) -> impl IntoResponse {
    log::info!("query sys_role_detail params: {:?}", &item);
    let rb = &state.batis;

    let result = Role::select_by_id(rb, &item.id).await;

    match result {
        Ok(d) => {
            let x = d.unwrap();

            let sys_role = QueryRoleDetailResp {
                id: x.id.unwrap(),                                 //主键
                role_name: x.role_name,                            //名称
                role_key: x.role_key,                              //角色权限字符串
                data_scope: x.data_scope, //数据范围（1：全部数据权限 2：自定数据权限 3：本部门数据权限 4：本部门及以下数据权限）
                status: x.status,         //状态(1:正常，0:禁用)
                sort: x.sort,             //排序
                remark: x.remark,         //备注
                del_flag: x.del_flag,     //删除标志（0代表删除 1代表存在）
                create_time: x.create_time.unwrap().0.to_string(), //创建时间
                update_time: x.update_time.unwrap().0.to_string(), //修改时间
            };

            BaseResponse::<QueryRoleDetailResp>::ok_result_data(sys_role)
        }
        Err(err) => BaseResponse::<QueryRoleDetailResp>::err_result_data(
            QueryRoleDetailResp::new(),
            err.to_string(),
        ),
    }
}

/*
 *查询角色信息列表
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn query_sys_role_list(
    State(state): State<Arc<AppState>>,
    Json(item): Json<QueryRoleListReq>,
) -> impl IntoResponse {
    log::info!("query_sys_role_list params: {:?}", &item);
    let rb = &state.batis;

    let role_name = item.role_name.as_deref().unwrap_or_default();
    let status_id = item.status_id.unwrap_or(2);

    let page = &PageRequest::new(item.page_no, item.page_size);
    let result = Role::select_page_by_name(rb, page, role_name, status_id).await;

    match result {
        Ok(d) => {
            let total = d.total;

            let mut sys_role_list_data: Vec<RoleListDataResp> = Vec::new();
            for x in d.records {
                sys_role_list_data.push(RoleListDataResp {
                    id: x.id.unwrap(),                                 //主键
                    role_name: x.role_name,                            //名称
                    role_key: x.role_key,                              //角色权限字符串
                    data_scope: x.data_scope, //数据范围（1：全部数据权限 2：自定数据权限 3：本部门数据权限 4：本部门及以下数据权限）
                    status: x.status,         //状态(1:正常，0:禁用)
                    sort: x.sort,             //排序
                    remark: x.remark,         //备注
                    del_flag: x.del_flag,     //删除标志（0代表删除 1代表存在）
                    create_time: x.create_time.unwrap().0.to_string(), //创建时间
                    update_time: x.update_time.unwrap().0.to_string(), //修改时间
                })
            }

            BaseResponse::ok_result_page(sys_role_list_data, total)
        }
        Err(err) => BaseResponse::err_result_page(RoleListDataResp::new(), err.to_string()),
    }
}

/*
 *查询角色关联的菜单
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn query_role_menu(
    State(state): State<Arc<AppState>>,
    Json(item): Json<QueryRoleMenuReq>,
) -> impl IntoResponse {
    log::info!("query_role_menu params: {:?}", &item);
    let rb = &state.batis;

    // 查询所有菜单
    let menu_list = Menu::select_all(rb).await.unwrap_or_default();

    let mut menu_data_list: Vec<MenuDataList> = Vec::new();
    let mut role_menu_ids: Vec<i64> = Vec::new();

    for y in menu_list {
        let x = y.clone();
        menu_data_list.push(MenuDataList {
            id: x.id.unwrap(),
            parent_id: x.parent_id,
            title: x.menu_name,
            key: y.id.unwrap().to_string(),
            label: y.menu_name,
            is_penultimate: y.parent_id == 2,
        });
        role_menu_ids.push(x.id.unwrap())
    }

    //不是超级管理员的时候,就要查询角色和菜单的关联
    if item.role_id != 1 {
        role_menu_ids.clear();
        let role_menu_list = query_menu_by_role(rb, item.role_id)
            .await
            .unwrap_or_default();

        for x in role_menu_list {
            let m_id = x.get("menu_id").unwrap().clone();
            role_menu_ids.push(m_id)
        }
    }

    BaseResponse::<QueryRoleMenuData>::ok_result_data(QueryRoleMenuData {
        menu_ids: role_menu_ids,
        menu_list: menu_data_list,
    })
}

/*
 *更新角色关联的菜单
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn update_role_menu(
    State(state): State<Arc<AppState>>,
    Json(item): Json<UpdateRoleMenuReq>,
) -> impl IntoResponse {
    log::info!("update_role_menu params: {:?}", &item);
    let role_id = item.role_id;

    let rb = &state.batis;

    let role_menu_result = RoleMenu::delete_by_column(rb, "role_id", &role_id).await;

    match role_menu_result {
        Ok(_) => {
            let mut menu_role: Vec<RoleMenu> = Vec::new();

            for id in &item.menu_ids {
                let menu_id = id.clone();
                menu_role.push(RoleMenu {
                    id: None,
                    create_time: Some(DateTime::now()),
                    menu_id,
                    role_id: role_id.clone(),
                })
            }

            let result = RoleMenu::insert_batch(rb, &menu_role, item.menu_ids.len() as u64).await;

            match result {
                Ok(_u) => BaseResponse::<String>::ok_result(),
                Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
            }
        }
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}
