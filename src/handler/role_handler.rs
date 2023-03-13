use std::sync::Arc;
use axum::extract::State;
use axum::Json;
use axum::response::IntoResponse;
use rbatis::rbdc::datetime::FastDateTime;
use rbatis::sql::{PageRequest};
use crate::{AppState};
use crate::model::entity::{SysRole, query_menu_by_role, SysMenu, SysMenuRole};
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
            let page_no = d.page_no;
            let page_size = d.page_size;

            let mut role_list: Vec<RoleListData> = Vec::new();

            for x in d.records {
                role_list.push(RoleListData {
                    id: x.id.unwrap(),
                    sort: x.sort.unwrap(),
                    status_id: x.status_id.unwrap(),
                    role_name: x.role_name.unwrap_or_default(),
                    remark: x.remark.unwrap_or_default(),
                    create_time: x.gmt_create.unwrap().0.to_string(),
                    update_time: x.gmt_modified.unwrap().0.to_string(),
                })
            }

            RoleListResp {
                msg: "successful".to_string(),
                code: 0,
                page_no,
                page_size,
                success: true,
                total,
                data: Some(role_list),
            }
        }
        Err(err) => {
            RoleListResp {
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


pub async fn role_save(State(state): State<Arc<AppState>>, Json(item): Json<RoleSaveReq>) -> impl IntoResponse {
    println!("model: {:?}", &item);
    let mut rb = &state.batis;

    let sys_role = SysRole {
        id: None,
        gmt_create: Some(FastDateTime::now()),
        gmt_modified: Some(FastDateTime::now()),
        status_id: Some(1),
        sort: Some(item.sort),
        role_name: Some(item.role_name),
        remark: Some(item.remark),
    };

    let result = SysRole::insert(&mut rb, &sys_role).await;

    Json(handle_result(result))
}


pub async fn role_update(State(state): State<Arc<AppState>>, Json(item): Json<RoleUpdateReq>) -> impl IntoResponse {
    println!("item: {:?}", &item);
    let mut rb = &state.batis;

    let sys_role = SysRole {
        id: Some(item.id),
        gmt_create: None,
        gmt_modified: Some(FastDateTime::now()),
        status_id: Some(item.status_id),
        sort: Some(item.sort),
        role_name: Some(item.role_name),
        remark: Some(item.remark),
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

    let role_menu_list = query_menu_by_role(&mut rb, item.role_id).await;

    let menu_list = SysMenu::select_all(&mut rb).await;

    let mut menu_data_list: Vec<MenuDataList> = Vec::new();
    let mut role_menus: Vec<i32> = Vec::new();


    for y in menu_list.unwrap_or_default() {
        let x = y.clone();
        menu_data_list.push(MenuDataList {
            id: x.id.unwrap(),
            parent_id: x.parent_id.unwrap(),
            title: x.menu_name.unwrap_or_default(),
            key: y.id.unwrap().to_string(),
            label: y.menu_name.unwrap_or_default(),
            is_penultimate: y.parent_id == Some(2)
        });
    }

    for x in role_menu_list.unwrap_or_default() {
        let m_id=x.get("menu_id").unwrap();
        role_menus.push(*m_id)
    }

    let resp = QueryRoleMenuResp {
        msg: "successful".to_string(),
        code: 0,
        data: QueryRoleMenuData {
            role_menus,
            menu_list: menu_data_list,
        },
    };

    Json(resp)
}


pub async fn update_role_menu(State(state): State<Arc<AppState>>, Json(item): Json<UpdateRoleMenuReq>) -> impl IntoResponse {
    log::info!("update_role_menu params: {:?}", &item);
    let role_id = item.role_id;

    let mut rb = &state.batis;

    SysMenuRole::delete_by_column(&mut rb, "role_id", &role_id).await.expect("删除角色菜单异常");

    let mut menu_role: Vec<SysMenuRole> = Vec::new();

    for x in &item.menu_ids {
        menu_role.push(SysMenuRole {
            id: None,
            gmt_create: Some(FastDateTime::now()),
            gmt_modified: Some(FastDateTime::now()),
            status_id: Some(1),
            sort: Some(1),
            menu_id: Some(*x),
            role_id: Some(role_id),
        })
    }

    let result = SysMenuRole::insert_batch(&mut rb, &menu_role, item.menu_ids.len() as u64).await;

    Json(handle_result(result))
}
