use crate::common::error::AppError;
use crate::common::result::{ok_result, ok_result_data, ok_result_page};
use crate::model::system::sys_menu_model::{select_count_menu_by_parent_id, Menu};
use crate::model::system::sys_role_menu_model::select_count_menu_by_menu_id;
use crate::vo::system::sys_menu_vo::*;
use crate::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use log::info;
use rbatis::PageRequest;
use rbs::value;
use std::sync::Arc;
/*
 *添加菜单信息
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn add_sys_menu(State(state): State<Arc<AppState>>, Json(mut item): Json<MenuReq>) -> impl IntoResponse {
    info!("add sys_menu params: {:?}", &item);
    let rb = &state.batis;

    if Menu::check_menu_name_unique(rb, &item.menu_name, None).await?.is_some() {
        return Err(AppError::BusinessError("菜单名称已存在"));
    }

    if let Some(url) = item.menu_url.clone() {
        if url != "".to_string() {
            if Menu::check_menu_url_unique(rb, &url, None).await?.is_some() {
                return Err(AppError::BusinessError("路由路径已存在"));
            }
        }
    }

    item.id = None;
    Menu::insert(rb, &Menu::from(item)).await.map(|_| ok_result())?
}

/*
 *删除菜单信息
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn delete_sys_menu(State(state): State<Arc<AppState>>, Json(item): Json<DeleteMenuReq>) -> impl IntoResponse {
    info!("delete sys_menu params: {:?}", &item);
    let rb = &state.batis;

    let ids = item.ids;
    for x in ids.clone() {
        if select_count_menu_by_parent_id(rb, &x).await? > 0 {
            return Err(AppError::BusinessError("存在子菜单,不允许删除"));
        }

        if select_count_menu_by_menu_id(rb, &x).await? > 0 {
            return Err(AppError::BusinessError("菜单已分配,不允许删除"));
        }
    }

    Menu::delete_by_map(rb, value! {"id": &ids}).await.map(|_| ok_result())?
}

/*
 *更新菜单信息
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn update_sys_menu(State(state): State<Arc<AppState>>, Json(item): Json<MenuReq>) -> impl IntoResponse {
    info!("update sys_menu params: {:?}", &item);
    let rb = &state.batis;

    let id = item.id;

    if id.is_none() {
        return Err(AppError::BusinessError("主键不能为空"));
    }

    if Menu::select_by_id(rb, &id.unwrap_or_default()).await?.is_none() {
        return Err(AppError::BusinessError("菜单信息不存在"));
    }

    if Menu::check_menu_name_unique(rb, &item.menu_name, id).await?.is_some() {
        return Err(AppError::BusinessError("菜单名称已存在"));
    }

    if let Some(url) = item.menu_url.clone() {
        if url != "".to_string() {
            if Menu::check_menu_url_unique(rb, &url, id).await?.is_some() {
                return Err(AppError::BusinessError("路由路径已存在"));
            }
        }
    }

    Menu::update_by_map(rb, &Menu::from(item), value! {"id": &id}).await.map(|_| ok_result())?
}

/*
 *更新菜单信息状态
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn update_sys_menu_status(State(state): State<Arc<AppState>>, Json(item): Json<UpdateMenuStatusReq>) -> impl IntoResponse {
    info!("update sys_menu_status params: {:?}", &item);
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
    info!("query sys_menu_detail params: {:?}", &item);
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
    info!("query sys_menu_list params: {:?}", &item);
    let rb = &state.batis;

    let menu_name = item.menu_name;
    let parent_id = item.parent_id;
    let status = item.status;

    Menu::query_sys_menu_list(rb, menu_name, parent_id, status)
        .await
        .map(|x| ok_result_data(x.into_iter().map(|x| x.into()).collect::<Vec<MenuResp>>()))?
}

/*
 *查询菜单信息(排除按钮)
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn query_sys_menu_list_simple(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let rb = &state.batis;

    Menu::select_menu_list(rb)
        .await
        .map(|x| ok_result_data(x.into_iter().map(|x| MenuSimpleResp::from(x)).collect::<Vec<MenuSimpleResp>>()))?
}

/*
 *查询菜单信息列表
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn query_sys_menu_resource_list(State(state): State<Arc<AppState>>, Json(item): Json<QueryMenuListReq>) -> impl IntoResponse {
    info!("query sys_menu_list params: {:?}", &item);
    let rb = &state.batis;

    let menu_name = item.menu_name;
    let parent_id = item.parent_id;
    let status = item.status;

    let page = &PageRequest::new(item.page_no, item.page_size);

    Menu::query_sys_menu_resource_list(rb, page, menu_name, parent_id, status)
        .await
        .map(|x| ok_result_page(x.records.into_iter().map(|x| x.into()).collect::<Vec<MenuResp>>(), x.total))?
}
