use axum::Json;
use axum::response::IntoResponse;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use diesel::associations::HasTable;
use log::{debug, error};

use crate::model::menu::{SysMenu, SysMenuAdd, SysMenuUpdate};
use crate::RB;
use crate::schema::sys_menu::{id, parent_id, sort, status_id};
use crate::schema::sys_menu::dsl::sys_menu;
use crate::vo::{err_result_msg, err_result_page, handle_result, ok_result_page};
use crate::vo::menu_vo::{*};

// 查询菜单
pub async fn menu_list(Json(item): Json<MenuListReq>) -> impl IntoResponse {
    log::info!("menu_list params: {:?}", &item);
    let mut menu_list: Vec<MenuListData> = Vec::new();
    match &mut RB.clone().get() {
        Ok(conn) => {
            let mut query = sys_menu::table().into_boxed();
            if let Some(i) = &item.status_id {
                query = query.filter(status_id.eq(i));
            }
            query = query.order(sort.asc());
            debug!("SQL:{}", diesel::debug_query::<diesel::mysql::Mysql, _>(&query).to_string());

            if let Ok(menus) = query.load::<SysMenu>(conn) {
                for menu in menus {
                    menu_list.push(MenuListData {
                        id: menu.id,
                        sort: menu.sort,
                        status_id: menu.status_id,
                        parent_id: menu.parent_id,
                        menu_name: menu.menu_name.clone(),
                        label: menu.menu_name,
                        menu_url: menu.menu_url,
                        icon: menu.menu_icon.unwrap_or_default(),
                        api_url: menu.api_url,
                        remark: menu.remark.unwrap_or_default(),
                        menu_type: menu.menu_type,
                        create_time: menu.create_time.to_string(),
                        update_time: menu.update_time.to_string(),
                    })
                }
            }

            Json(ok_result_page(menu_list, 0))
        }
        Err(err) => {
            error!("err:{}", err.to_string());
            Json(err_result_page(menu_list, err.to_string()))
        }
    }
}

// 添加菜单
pub async fn menu_save(Json(menu): Json<MenuSaveReq>) -> impl IntoResponse {
    log::info!("menu_save params: {:?}", &menu);

    let menu_add = SysMenuAdd {
        status_id: menu.status_id,
        sort: menu.sort,
        parent_id: menu.parent_id,
        menu_name: menu.menu_name,
        menu_url: menu.menu_url,
        api_url: menu.api_url,
        menu_icon: menu.icon,
        remark: menu.remark,
        menu_type: menu.menu_type,
    };

    let resp = match &mut RB.clone().get() {
        Ok(conn) => {
            handle_result(diesel::insert_into(sys_menu::table()).values(menu_add).execute(conn))
        }
        Err(err) => {
            error!("err:{}", err.to_string());
            err_result_msg(err.to_string())
        }
    };

    Json(resp)
}

// 更新菜单
pub async fn menu_update(Json(menu): Json<MenuUpdateReq>) -> impl IntoResponse {
    log::info!("menu_update params: {:?}", &menu);

    let s_menu = SysMenuUpdate {
        id: menu.id,
        status_id: menu.status_id,
        sort: menu.sort,
        parent_id: menu.parent_id,
        menu_name: menu.menu_name,
        menu_url: menu.menu_url,
        api_url: menu.api_url,
        menu_icon: menu.icon,
        remark: menu.remark,
        menu_type: menu.menu_type,
    };

    let resp = match &mut RB.clone().get() {
        Ok(conn) => {
            handle_result(diesel::update(sys_menu).filter(id.eq(&menu.id)).set(s_menu).execute(conn))
        }
        Err(err) => {
            error!("err:{}", err.to_string());
            err_result_msg(err.to_string())
        }
    };

    Json(resp)
}

// 删除菜单信息
pub async fn menu_delete(Json(item): Json<MenuDeleteReq>) -> impl IntoResponse {
    log::info!("menu_delete params: {:?}", &item);
    let resp = match &mut RB.clone().get() {
        Ok(conn) => {
            match sys_menu.filter(parent_id.eq(&item.id)).count().get_result::<i64>(conn) {
                Ok(count) => {
                    if count > 0 {
                        error!("err:{}", "有下级菜单,不能直接删除".to_string());
                        return Json(err_result_msg("有下级菜单,不能直接删除".to_string()));
                    }
                    handle_result(diesel::delete(sys_menu.filter(id.eq(&item.id))).execute(conn))
                }
                Err(err) => {
                    error!("err:{}", err.to_string());
                    err_result_msg(err.to_string())
                }
            }
        }
        Err(err) => {
            error!("err:{}", err.to_string());
            err_result_msg(err.to_string())
        }
    };

    Json(resp)
}