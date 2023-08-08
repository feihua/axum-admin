use std::sync::Arc;
use axum::extract::State;
use axum::Json;
use axum::response::IntoResponse;
use rbatis::rbdc::datetime::DateTime;
use rbatis::sql::{PageRequest};
use crate::{AppState};
use crate::model::role::{SysRole};
use crate::model::menu::{SysMenu};
use crate::model::role_menu::{query_menu_by_role, SysRoleMenu};
use crate::vo::handle_result;
use crate::vo::role_vo::*;


pub async fn role_list(State(state): State<Arc<AppState>>, Json(item): Json<RoleListReq>) -> impl IntoResponse {
    log::info!("role_list params: {:?}", &item);
    let mut rb = &state.batis;

    let role_name = item.role_name.as_deref().unwrap_or_default();
    let status_id = item.status_id.as_deref().unwrap_or_default();

    let page=&PageRequest::new(item.page_no, item.page_size);
    let result = SysRole::select_page_by_name(&mut rb, page,role_name,status_id).await;

    let resp = match result {
        Ok(d) => {
            let total = d.total;

            let mut role_list: Vec<RoleListData> = Vec::new();

            for x in d.records {
                role_list.push(RoleListData {
                    id: x.id.unwrap(),
                    sort: x.sort,
                    status_id: x.status_id,
                    role_name: x.role_name,
                    remark: x.remark.unwrap_or_default(),
                    create_time: x.create_time.unwrap().0.to_string(),
                    update_time: x.update_time.unwrap().0.to_string(),
                })
            }

            RoleListResp {
                msg: "successful".to_string(),
                code: 0,
                success: true,
                total,
                data: Some(role_list),
            }
        }
        Err(err) => {
            RoleListResp {
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


pub async fn role_save(State(state): State<Arc<AppState>>, Json(item): Json<RoleSaveReq>) -> impl IntoResponse {
    println!("model: {:?}", &item);
    let mut rb = &state.batis;

    let sys_role = SysRole {
        id: None,
        create_time: Some(DateTime::now()),
        update_time: Some(DateTime::now()),
        status_id: item.status_id,
        sort: item.sort,
        role_name: item.role_name,
        remark: item.remark,
    };

    let result = SysRole::insert(&mut rb, &sys_role).await;

    Json(handle_result(result))
}


pub async fn role_update(State(state): State<Arc<AppState>>, Json(item): Json<RoleUpdateReq>) -> impl IntoResponse {
    println!("item: {:?}", &item);
    let mut rb = &state.batis;

    let sys_role = SysRole {
        id: Some(item.id),
        create_time: None,
        update_time: Some(DateTime::now()),
        status_id: item.status_id,
        sort: item.sort,
        role_name: item.role_name,
        remark: item.remark,
    };

    let result = SysRole::update_by_column(&mut rb, &sys_role, "id").await;

    Json(handle_result(result))
}


pub async fn role_delete(State(state): State<Arc<AppState>>, Json(item): Json<RoleDeleteReq>) -> impl IntoResponse {
    println!("item: {:?}", &item);
    let mut rb = &state.batis;

    let result = SysRole::delete_in_column(&mut rb, "id", &item.ids).await;

    Json(handle_result(result))
}


pub async fn query_role_menu(State(state): State<Arc<AppState>>, Json(item): Json<QueryRoleMenuReq>) -> impl IntoResponse {
    log::info!("query_role_menu params: {:?}", &item);
    let mut rb = &state.batis;

    // 查询所有菜单
    let menu_list = SysMenu::select_all(&mut rb).await.unwrap_or_default();

    let mut menu_data_list: Vec<MenuDataList> = Vec::new();
    let mut role_menu_ids: Vec<i32> = Vec::new();

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
        let role_menu_list = query_menu_by_role(&mut rb, item.role_id).await.unwrap_or_default();

        for x in role_menu_list {
            let m_id = x.get("menu_id").unwrap().clone();
            role_menu_ids.push(m_id)
        }
    }

    let resp = QueryRoleMenuResp {
        msg: "successful".to_string(),
        code: 0,
        data: QueryRoleMenuData {
            role_menus: role_menu_ids,
            menu_list: menu_data_list,
        },
    };

    Json(resp)
}


pub async fn update_role_menu(State(state): State<Arc<AppState>>, Json(item): Json<UpdateRoleMenuReq>) -> impl IntoResponse {
    log::info!("update_role_menu params: {:?}", &item);
    let role_id = item.role_id;

    let mut rb = &state.batis;

    SysRoleMenu::delete_by_column(&mut rb, "role_id", &role_id).await.expect("删除角色菜单异常");

    let mut menu_role: Vec<SysRoleMenu> = Vec::new();

    for x in &item.menu_ids {
        let menu_id = x.clone();
        menu_role.push(SysRoleMenu {
            id: None,
            create_time: Some(DateTime::now()),
            update_time: Some(DateTime::now()),
            status_id: 1,
            sort: 1,
            menu_id,
            role_id: role_id.clone(),
        })
    }

    let result = SysRoleMenu::insert_batch(&mut rb, &menu_role, item.menu_ids.len() as u64).await;

    Json(handle_result(result))
}
