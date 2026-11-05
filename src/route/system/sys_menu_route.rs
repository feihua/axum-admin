use crate::AppState;
use crate::handler::system::sys_menu_handler;
use axum::Router;
use route_macro::routes;
use std::sync::Arc;
/*
 *构建菜单信息路由
 *author：刘飞华
 *date：2024/12/12 17:04:49
 */
pub fn build_sys_menu_route() -> Router<Arc<AppState>> {
    routes! {
        "/system/menu/addMenu"=>post(sys_menu_handler::add_sys_menu),
        "/system/menu/deleteMenu"=>post(sys_menu_handler::delete_sys_menu),
        "/system/menu/updateMenu"=>post(sys_menu_handler::update_sys_menu),
        "/system/menu/updateMenuStatus"=>post(sys_menu_handler::update_sys_menu_status),
        "/system/menu/queryMenuDetail"=>post(sys_menu_handler::query_sys_menu_detail),
        "/system/menu/queryMenuList"=>post(sys_menu_handler::query_sys_menu_list),
        "/system/menu/queryMenuListSimple"=>post(sys_menu_handler::query_sys_menu_list_simple),
        "/system/menu/queryMenuResourceList"=>post(sys_menu_handler::query_sys_menu_resource_list),
    }
}
