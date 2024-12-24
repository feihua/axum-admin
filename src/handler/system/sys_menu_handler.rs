use crate::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use rbs::to_value;
use std::sync::Arc;

use crate::common::result::BaseResponse;
use crate::model::system::sys_menu_model::Menu;
use crate::vo::system::sys_menu_vo::*;

/*
 *添加菜单信息
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn add_sys_menu(
    State(state): State<Arc<AppState>>,
    Json(item): Json<AddMenuReq>,
) -> impl IntoResponse {
    log::info!("add sys_menu params: {:?}", &item);
    let rb = &state.batis;

    let sys_menu = Menu {
        id: None,                               //主键
        menu_name: item.menu_name,              //菜单名称
        menu_type: item.menu_type,              //菜单类型(1：目录   2：菜单   3：按钮)
        status: item.status,                    //状态(1:正常，0:禁用)
        sort: item.sort,                        //排序
        parent_id: item.parent_id.unwrap_or(0), //父ID
        menu_url: item.menu_url,                //路由路径
        api_url: item.api_url,                  //接口URL
        menu_icon: item.menu_icon,              //菜单图标
        remark: item.remark,                    //备注
        create_time: None,                      //创建时间
        update_time: None,                      //修改时间
    };

    let result = Menu::insert(rb, &sys_menu).await;

    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

/*
 *删除菜单信息
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn delete_sys_menu(
    State(state): State<Arc<AppState>>,
    Json(item): Json<DeleteMenuReq>,
) -> impl IntoResponse {
    log::info!("delete sys_menu params: {:?}", &item);
    let rb = &state.batis;

    //有下级的时候 不能直接删除
    let menus = Menu::select_by_column(rb, "parent_id", &item.id)
        .await
        .unwrap_or_default();

    if menus.len() > 0 {
        return BaseResponse::<String>::err_result_msg("有下级菜单,不能直接删除".to_string());
    }

    let result = Menu::delete_by_column(rb, "id", &item.id).await;

    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

/*
 *更新菜单信息
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn update_sys_menu(
    State(state): State<Arc<AppState>>,
    Json(item): Json<UpdateMenuReq>,
) -> impl IntoResponse {
    log::info!("update sys_menu params: {:?}", &item);
    let rb = &state.batis;

    let sys_menu = Menu {
        id: Some(item.id),         //主键
        menu_name: item.menu_name, //菜单名称
        menu_type: item.menu_type, //菜单类型(1：目录   2：菜单   3：按钮)
        status: item.status,       //状态(1:正常，0:禁用)
        sort: item.sort,           //排序
        parent_id: item.parent_id, //父ID
        menu_url: item.menu_url,   //路由路径
        api_url: item.api_url,     //接口URL
        menu_icon: item.menu_icon, //菜单图标
        remark: item.remark,       //备注
        create_time: None,         //创建时间
        update_time: None,         //修改时间
    };

    let result = Menu::update_by_column(rb, &sys_menu, "id").await;

    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

/*
 *更新菜单信息状态
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn update_sys_menu_status(
    State(state): State<Arc<AppState>>,
    Json(item): Json<UpdateMenuStatusReq>,
) -> impl IntoResponse {
    log::info!("update sys_menu_status params: {:?}", &item);
    let rb = &state.batis;

    let update_sql = format!(
        "update sys_menu set status = ? where id in ({})",
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
 *查询菜单信息详情
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn query_sys_menu_detail(
    State(state): State<Arc<AppState>>,
    Json(item): Json<QueryMenuDetailReq>,
) -> impl IntoResponse {
    log::info!("query sys_menu_detail params: {:?}", &item);
    let rb = &state.batis;

    let result = Menu::select_by_id(rb, &item.id).await;

    match result {
        Ok(d) => {
            let x = d.unwrap();

            let sys_menu = QueryMenuDetailResp {
                id: x.id.unwrap(),                                 //主键
                menu_name: x.menu_name,                            //菜单名称
                menu_type: x.menu_type, //菜单类型(1：目录   2：菜单   3：按钮)
                status: x.status,       //状态(1:正常，0:禁用)
                sort: x.sort,           //排序
                parent_id: x.parent_id, //父ID
                menu_url: x.menu_url.unwrap_or_default(), //路由路径
                api_url: x.api_url.unwrap_or_default(), //接口URL
                menu_icon: x.menu_icon.unwrap_or_default(), //菜单图标
                remark: x.remark.unwrap_or_default(), //备注
                create_time: x.create_time.unwrap().0.to_string(), //创建时间
                update_time: x.update_time.unwrap().0.to_string(), //修改时间
            };

            BaseResponse::<QueryMenuDetailResp>::ok_result_data(sys_menu)
        }
        Err(err) => BaseResponse::<QueryMenuDetailResp>::err_result_data(
            QueryMenuDetailResp::new(),
            err.to_string(),
        ),
    }
}

/*
 *查询菜单信息列表
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn query_sys_menu_list(
    State(state): State<Arc<AppState>>,
    Json(item): Json<QueryMenuListReq>,
) -> impl IntoResponse {
    log::info!("query sys_menu_list params: {:?}", &item);
    let rb = &state.batis;

    let result = Menu::select_all(rb).await;

    match result {
        Ok(d) => {
            let mut sys_menu_list_data: Vec<MenuListDataResp> = Vec::new();
            for x in d {
                sys_menu_list_data.push(MenuListDataResp {
                    id: x.id.unwrap(),                                 //主键
                    menu_name: x.menu_name,                            //菜单名称
                    menu_type: x.menu_type, //菜单类型(1：目录   2：菜单   3：按钮)
                    status: x.status,       //状态(1:正常，0:禁用)
                    sort: x.sort,           //排序
                    parent_id: x.parent_id, //父ID
                    menu_url: x.menu_url.unwrap_or_default(), //路由路径
                    api_url: x.api_url.unwrap_or_default(), //接口URL
                    menu_icon: x.menu_icon.unwrap_or_default(), //菜单图标
                    remark: x.remark.unwrap_or_default(), //备注
                    create_time: x.create_time.unwrap().0.to_string(), //创建时间
                    update_time: x.update_time.unwrap().0.to_string(), //修改时间
                })
            }

            BaseResponse::ok_result_page(sys_menu_list_data, 0)
        }
        Err(err) => BaseResponse::err_result_page(MenuListDataResp::new(), err.to_string()),
    }
}
