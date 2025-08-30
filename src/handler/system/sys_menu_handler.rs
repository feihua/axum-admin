use crate::common::error::AppError;
use crate::common::result::{ok_result, ok_result_data};
use crate::model::system::sys_menu_model::{select_count_menu_by_parent_id, Menu};
use crate::model::system::sys_role_menu_model::select_count_menu_by_menu_id;
use crate::vo::system::sys_menu_vo::*;
use crate::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use rbatis::rbdc::DateTime;
use rbs::value;
use std::sync::Arc;
/*
 *添加菜单信息
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn add_sys_menu(State(state): State<Arc<AppState>>, Json(item): Json<MenuReq>) -> impl IntoResponse {
    log::info!("add sys_menu params: {:?}", &item);
    let rb = &state.batis;

    if Menu::select_by_menu_name(rb, &item.menu_name).await?.is_some() {
        return Err(AppError::BusinessError("菜单名称已存在"));
    }

    let menu_url = item.menu_url.clone();
    if menu_url.is_some() {
        if Menu::select_by_menu_url(rb, &menu_url.unwrap()).await?.is_some() {
            return Err(AppError::BusinessError("路由路径已存在"));
        }
    }

    Menu::insert(rb, &Menu::from(item)).await.map(|_| ok_result())?
}

/*
 *删除菜单信息
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn delete_sys_menu(State(state): State<Arc<AppState>>, Json(item): Json<DeleteMenuReq>) -> impl IntoResponse {
    log::info!("delete sys_menu params: {:?}", &item);
    let rb = &state.batis;

    //有下级的时候 不能直接删除

    if select_count_menu_by_parent_id(rb, &item.id).await? > 0 {
        return Err(AppError::BusinessError("存在子菜单,不允许删除"));
    }

    if select_count_menu_by_menu_id(rb, &item.id).await? > 0 {
        return Err(AppError::BusinessError("菜单已分配,不允许删除"));
    }

    Menu::delete_by_map(rb, value! {"id": &item.id}).await.map(|_| ok_result())?
}

/*
 *更新菜单信息
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn update_sys_menu(State(state): State<Arc<AppState>>, Json(item): Json<MenuReq>) -> impl IntoResponse {
    log::info!("update sys_menu params: {:?}", &item);
    let rb = &state.batis;

    let id = item.id;
    if Menu::select_by_id(rb, &id.unwrap_or_default()).await?.is_none() {
        return Err(AppError::BusinessError("菜单信息不存在"));
    }

    if let Some(x) = Menu::select_by_menu_name(rb, &item.menu_name).await? {
        if x.id != item.id {
            return Err(AppError::BusinessError("菜单名称已存在"));
        }
    }

    let menu_url = item.menu_url.clone();
    if menu_url.is_some() {
        if let Some(x) = Menu::select_by_menu_url(rb, &menu_url.unwrap()).await? {
            if x.id != item.id {
                return Err(AppError::BusinessError("路由路径已存在"));
            }
        }
    }

    let mut data = Menu::from(item);
    data.update_time = Some(DateTime::now());
    Menu::update_by_map(rb, &data, value! {"id": &id}).await.map(|_| ok_result())?
}

/*
 *更新菜单信息状态
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn update_sys_menu_status(State(state): State<Arc<AppState>>, Json(item): Json<UpdateMenuStatusReq>) -> impl IntoResponse {
    log::info!("update sys_menu_status params: {:?}", &item);
    let rb = &state.batis;

    let ids = item.ids.iter().map(|_| "?").collect::<Vec<&str>>().join(", ");
    let update_sql = format!("update sys_menu set status = ? where id in ({})", ids);

    let mut param = vec![value!(item.status)];
    param.extend(item.ids.iter().map(|&id| value!(id)));
    rb.exec(&update_sql, param).await.map(|_| ok_result())?
}

/*
 *查询菜单信息详情
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn query_sys_menu_detail(State(state): State<Arc<AppState>>, Json(item): Json<QueryMenuDetailReq>) -> impl IntoResponse {
    log::info!("query sys_menu_detail params: {:?}", &item);
    let rb = &state.batis;

    Menu::select_by_id(rb, &item.id).await?.map_or_else(
        || Err(AppError::BusinessError("菜单信息不存在")),
        |x| {
            let data: MenuResp = x.into();
            ok_result_data(data)
        },
    )
}

/*
 *查询菜单信息列表
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn query_sys_menu_list(State(state): State<Arc<AppState>>, Json(item): Json<QueryMenuListReq>) -> impl IntoResponse {
    log::info!("query sys_menu_list params: {:?}", &item);
    let rb = &state.batis;

    Menu::select_all(rb).await.map(|x| ok_result_data(x.into_iter().map(|x| x.into()).collect::<Vec<MenuResp>>()))?
}

/*
 *查询菜单信息(排除按钮)
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn query_sys_menu_list_simple(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let rb = &state.batis;

    let list = Menu::select_menu_list(rb).await?;

    let mut menu_list: Vec<MenuListSimpleDataResp> = Vec::new();
    for x in list {
        menu_list.push(MenuListSimpleDataResp {
            id: x.id,               //主键
            menu_name: x.menu_name, //菜单名称
            parent_id: x.parent_id, //父ID
        })
    }

    ok_result_data(menu_list)
}
