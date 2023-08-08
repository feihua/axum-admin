use std::sync::Arc;
use axum::extract::State;
use axum::Json;
use axum::response::IntoResponse;
use rbatis::rbdc::datetime::DateTime;
use rbatis::sql::{PageRequest};
use crate::{AppState};

use crate::model::menu::{SysMenu};
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
                    sort: x.sort,
                    status_id: x.status_id,
                    parent_id: x.parent_id,
                    menu_name: x.menu_name.clone(),
                    label: x.menu_name,
                    menu_url: x.menu_url.unwrap_or_default(),
                    icon: x.menu_icon.unwrap_or_default(),
                    api_url: x.api_url.unwrap_or_default(),
                    remark: x.remark.unwrap_or_default(),
                    menu_type: x.menu_type,
                    create_time: x.create_time.unwrap().0.to_string(),
                    update_time: x.update_time.unwrap().0.to_string(),
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
        create_time: Some(DateTime::now()),
        update_time: Some(DateTime::now()),
        status_id: item.status_id,
        sort: item.sort,
        parent_id: item.parent_id.unwrap_or(0),
        menu_name: item.menu_name,
        menu_url: item.menu_url,
        api_url: item.api_url,
        menu_icon: item.icon,
        remark: item.remark,
        menu_type: item.menu_type,
    };

    let result = SysMenu::insert(&mut rb, &role).await;

    Json(handle_result(result))
}

pub async fn menu_update(State(state): State<Arc<AppState>>, Json(item): Json<MenuUpdateReq>) -> impl IntoResponse {
    log::info!("menu_update params: {:?}", &item);
    let mut rb = &state.batis;

    let sys_menu = SysMenu {
        id: Some(item.id),
        create_time: None,
        update_time: Some(DateTime::now()),
        status_id: item.status_id,
        sort: item.sort,
        parent_id: item.parent_id,
        menu_name: item.menu_name,
        menu_url: item.menu_url,
        api_url: item.api_url,
        menu_icon: item.icon,
        remark: item.remark,
        menu_type: item.menu_type,
    };

    let result = SysMenu::update_by_column(&mut rb, &sys_menu, "id").await;

    Json(handle_result(result))
}


pub async fn menu_delete(State(state): State<Arc<AppState>>, Json(item): Json<MenuDeleteReq>) -> impl IntoResponse {
    log::info!("menu_delete params: {:?}", &item);
    let mut rb = &state.batis;

    let result = SysMenu::delete_by_column(&mut rb, "id", &item.id).await;

    Json(handle_result(result))
}