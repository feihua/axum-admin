use std::sync::Arc;
use axum::extract::State;
use axum::Json;
use axum::response::IntoResponse;
use rbatis::rbdc::datetime::FastDateTime;
use rbatis::sql::{PageRequest};
use crate::{AppState};

use crate::model::entity::{SysMenu};
use crate::vo::handle_result;
use crate::vo::menu_vo::{*};


pub async fn menu_list(State(state): State<Arc<AppState>>, Json(item): Json<MenuListReq>) -> impl IntoResponse {
    log::info!("menu_list params: {:?}", &item);
    let mut rb = &state.batis;

    let result = SysMenu::select_page(&mut rb, &PageRequest::new(1, 1000)).await;

    let resp = match result {
        Ok(d) => {
            let total = d.total;

            let mut menu_list: Vec<MenuListData> = Vec::new();

            for x in d.records {
                menu_list.push(MenuListData {
                    id: x.id.unwrap(),
                    sort: x.sort.unwrap(),
                    status_id: x.status_id.unwrap(),
                    parent_id: x.parent_id.unwrap(),
                    menu_name: x.menu_name.as_ref().unwrap().to_string(),
                    label: x.menu_name.unwrap_or_default(),
                    menu_url: x.menu_url.unwrap_or_default(),
                    icon: x.menu_icon.unwrap_or_default(),
                    api_url: x.api_url.unwrap_or_default(),
                    remark: x.remark.unwrap_or_default(),
                    menu_type: x.menu_type.unwrap(),
                    create_time: x.gmt_create.unwrap().0.to_string(),
                    update_time: x.gmt_modified.unwrap().0.to_string(),
                })
            }
            MenuListResp {
                msg: "successful".to_string(),
                code: 0,
                total,
                data: Some(menu_list),
            }
        }
        Err(err) => {
            MenuListResp {
                msg: err.to_string(),
                code: 1,
                total: 0,
                data: None,
            }
        }
    };

    Json(resp)
}

pub async fn menu_save(State(state): State<Arc<AppState>>, Json(item): Json<MenuSaveReq>) -> impl IntoResponse {
    log::info!("menu_save params: {:?}", &item);
    let mut rb = &state.batis;

    let role = SysMenu {
        id: None,
        gmt_create: Some(FastDateTime::now()),
        gmt_modified: Some(FastDateTime::now()),
        status_id: Some(item.status_id),
        sort: Some(item.sort),
        parent_id: Some(item.parent_id.unwrap_or(0)),
        menu_name: Some(item.menu_name),
        menu_url: Some(item.menu_url),
        api_url: Some(item.api_url),
        menu_icon: Some(item.icon),
        remark: Some(item.remark),
        menu_type: Some(item.menu_type),
    };

    let result = SysMenu::insert(&mut rb, &role).await;

    Json(handle_result(result))
}

pub async fn menu_update(State(state): State<Arc<AppState>>, Json(item): Json<MenuUpdateReq>) -> impl IntoResponse {
    log::info!("menu_update params: {:?}", &item);
    let mut rb = &state.batis;

    let sys_menu = SysMenu {
        id: Some(item.id),
        gmt_create: None,
        gmt_modified: Some(FastDateTime::now()),
        status_id: Some(item.status_id),
        sort: Some(item.sort),
        parent_id: Some(item.parent_id),
        menu_name: Some(item.menu_name),
        menu_url: Some(item.menu_url),
        api_url: Some(item.api_url),
        menu_icon: Some(item.icon),
        remark: Some(item.remark),
        menu_type: Some(item.menu_type),
    };

    let result = SysMenu::update_by_column(&mut rb, &sys_menu, "id").await;

    Json(handle_result(result))
}


pub async fn menu_delete(State(state): State<Arc<AppState>>, Json(item): Json<MenuDeleteReq>) -> impl IntoResponse {
    log::info!("menu_delete params: {:?}", &item);
    let mut rb = &state.batis;

    let result = SysMenu::delete_in_column(&mut rb, "id", &item.ids).await;

    Json(handle_result(result))
}