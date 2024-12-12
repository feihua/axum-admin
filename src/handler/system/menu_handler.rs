use std::sync::Arc;

use axum::extract::State;
use axum::Json;
use axum::response::IntoResponse;
use rbatis::rbdc::datetime::DateTime;

use crate::AppState;
use crate::common::result::BaseResponse;
use crate::model::system::menu::SysMenu;
use crate::vo::system::menu_vo::{*};

// 添加菜单
pub async fn menu_save(State(state): State<Arc<AppState>>, Json(item): Json<MenuSaveReq>) -> impl IntoResponse {
    log::info!("menu_save params: {:?}", &item);
    let rb = &state.batis;

    let sys_menu = SysMenu {
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

    let result = SysMenu::insert(rb, &sys_menu).await;

    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

// 删除菜单信息
pub async fn menu_delete(State(state): State<Arc<AppState>>, Json(item): Json<MenuDeleteReq>) -> impl IntoResponse {
    log::info!("menu_delete params: {:?}", &item);
    let rb = &state.batis;

    //有下级的时候 不能直接删除
    let menus = SysMenu::select_by_column(rb, "parent_id", &item.id).await.unwrap_or_default();

    if menus.len() > 0 {
        return BaseResponse::<String>::err_result_msg("有下级菜单,不能直接删除".to_string());
    }

    let result = SysMenu::delete_by_column(rb, "id", &item.id).await;

    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

// 更新菜单
pub async fn menu_update(State(state): State<Arc<AppState>>, Json(item): Json<MenuUpdateReq>) -> impl IntoResponse {
    log::info!("menu_update params: {:?}", &item);
    let rb = &state.batis;

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

    let result = SysMenu::update_by_column(rb, &sys_menu, "id").await;

    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

// 查询菜单
pub async fn menu_list(State(state): State<Arc<AppState>>, Json(item): Json<MenuListReq>) -> impl IntoResponse {
    log::info!("menu_list params: {:?}", &item);
    let rb = &state.batis;

    // 菜单是树形结构不需要分页
    let result = SysMenu::select_all(rb).await;

    let mut menu_list: Vec<MenuListData> = Vec::new();
    match result {
        Ok(sys_menu_list) => {
            for menu in sys_menu_list {
                menu_list.push(MenuListData {
                    id: menu.id.unwrap(),
                    sort: menu.sort,
                    status_id: menu.status_id,
                    parent_id: menu.parent_id,
                    menu_name: menu.menu_name.clone(),
                    label: menu.menu_name,
                    menu_url: menu.menu_url.unwrap_or_default(),
                    icon: menu.menu_icon.unwrap_or_default(),
                    api_url: menu.api_url.unwrap_or_default(),
                    remark: menu.remark.unwrap_or_default(),
                    menu_type: menu.menu_type,
                    create_time: menu.create_time.unwrap().0.to_string(),
                    update_time: menu.update_time.unwrap().0.to_string(),
                })
            }
            BaseResponse::ok_result_page(menu_list, 0)
        }
        Err(err) => {
            BaseResponse::err_result_page(menu_list, err.to_string())
        }
    }
}
