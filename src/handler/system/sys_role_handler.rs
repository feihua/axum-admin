use crate::common::error::AppError;
use crate::common::result::{ok_result, ok_result_data, ok_result_page};
use crate::model::system::sys_menu_model::Menu;
use crate::model::system::sys_role_dept_model::RoleDept;
use crate::model::system::sys_role_menu_model::{query_menu_by_role, RoleMenu};
use crate::model::system::sys_role_model::Role;
use crate::model::system::sys_user_model::{count_allocated_list, count_unallocated_list, select_allocated_list, select_unallocated_list};
use crate::model::system::sys_user_role_model::{count_user_role_by_role_id, delete_user_role_by_role_id_user_id, UserRole};
use crate::vo::system::sys_role_vo::*;
use crate::vo::system::sys_user_vo::UserResp;
use crate::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use log::info;
use rbatis::plugin::page::PageRequest;
use rbatis::rbdc::DateTime;
use rbs::value;
use std::sync::Arc;
/*
 *添加角色信息
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn add_sys_role(State(state): State<Arc<AppState>>, Json(mut item): Json<RoleReq>) -> impl IntoResponse {
    info!("add sys_role params: {:?}", &item);
    let rb = &state.batis;

    if Role::select_by_role_name(rb, &item.role_name).await?.is_some() {
        return Err(AppError::BusinessError("角色名称已存在"));
    }

    if Role::select_by_role_key(rb, &item.role_key).await?.is_some() {
        return Err(AppError::BusinessError("角色权限已存在"));
    }

    item.id = None;
    Role::insert(rb, &Role::from(item)).await.map(|_| ok_result())?
}

/*
 *删除角色信息
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn delete_sys_role(State(state): State<Arc<AppState>>, Json(item): Json<DeleteRoleReq>) -> impl IntoResponse {
    info!("delete sys_role params: {:?}", &item);
    let rb = &state.batis;

    let ids = item.ids.clone();

    if ids.contains(&1) {
        return Err(AppError::BusinessError("不允许操作超级管理员角色"));
    }

    for id in ids {
        if let None = Role::select_by_id(rb, &id).await? {
            return Err(AppError::BusinessError("角色不存在,不能删除"));
        }

        if count_user_role_by_role_id(rb, id).await? > 0 {
            return Err(AppError::BusinessError("分配,不能删除"));
        }
    }

    RoleMenu::delete_by_map(rb, value! {"role_id": &item.ids}).await?;
    RoleDept::delete_by_map(rb, value! {"role_id": &item.ids}).await?;
    Role::delete_by_map(rb, value! {"id": &item.ids}).await.map(|_| ok_result())?
}

/*
 *更新角色信息
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn update_sys_role(State(state): State<Arc<AppState>>, Json(item): Json<RoleReq>) -> impl IntoResponse {
    info!("update sys_role params: {:?}", &item);
    let rb = &state.batis;

    let id = item.id;
    if item.id.is_none() {
        return Err(AppError::BusinessError("主键不能为空"));
    }

    if id == Some(1) {
        return Err(AppError::BusinessError("不允许操作超级管理员角色"));
    }

    if Role::select_by_id(rb, &id.unwrap_or_default()).await?.is_none() {
        return Err(AppError::BusinessError("角色不存在"));
    }

    if let Some(x) = Role::select_by_role_name(rb, &item.role_name).await? {
        if x.id != id {
            return Err(AppError::BusinessError("角色名称已存在"));
        }
    }

    if let Some(x) = Role::select_by_role_key(rb, &item.role_key).await? {
        if x.id != id {
            return Err(AppError::BusinessError("角色权限已存在"));
        }
    }

    Role::update_by_map(rb, &Role::from(item), value! {"id": &id}).await.map(|_| ok_result())?
}

/*
 *更新角色信息状态
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn update_sys_role_status(State(state): State<Arc<AppState>>, Json(item): Json<UpdateRoleStatusReq>) -> impl IntoResponse {
    info!("update sys_role_status params: {:?}", &item);
    let rb = &state.batis;

    if item.ids.contains(&1) {
        return Err(AppError::BusinessError("不允许操作超级管理员角色"));
    }

    let update_sql = format!("update sys_role set status = ? where id in ({})", item.ids.iter().map(|_| "?").collect::<Vec<&str>>().join(", "));

    let mut param = vec![value!(item.status)];
    param.extend(item.ids.iter().map(|&id| value!(id)));
    rb.exec(&update_sql, param).await.map(|_| ok_result())?
}

/*
 *查询角色信息详情
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn query_sys_role_detail(State(state): State<Arc<AppState>>, Json(item): Json<QueryRoleDetailReq>) -> impl IntoResponse {
    info!("query sys_role_detail params: {:?}", &item);
    let rb = &state.batis;

    Role::select_by_id(rb, &item.id).await?.map_or_else(
        || Err(AppError::BusinessError("角色不存在")),
        |x| {
            let data: RoleResp = x.into();
            ok_result_data(data)
        },
    )
}

/*
 *查询角色信息列表
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn query_sys_role_list(State(state): State<Arc<AppState>>, Json(item): Json<QueryRoleListReq>) -> impl IntoResponse {
    info!("query sys_role_list params: {:?}", &item);
    let rb = &state.batis;

    let page = &PageRequest::new(item.page_no, item.page_size);

    Role::select_sys_role_list(rb, page, &item)
        .await
        .map(|x| ok_result_page(x.records.into_iter().map(|x| x.into()).collect::<Vec<RoleResp>>(), x.total))?
}

/*
 *查询角色关联的菜单
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn query_role_menu(State(state): State<Arc<AppState>>, Json(item): Json<QueryRoleMenuReq>) -> impl IntoResponse {
    info!("query role_menu params: {:?}", &item);
    let rb = &state.batis;

    // 查询所有菜单
    let menu_list_all = Menu::select_all(rb).await?;

    let mut menu_list: Vec<MenuDataList> = Vec::new();
    let mut menu_ids: Vec<Option<i64>> = Vec::new();

    for y in menu_list_all {
        let x = y.clone();
        menu_list.push(MenuDataList {
            id: x.id,
            parent_id: x.parent_id,
            title: x.menu_name,
            key: y.id.unwrap_or_default().to_string(),
            label: y.menu_name,
            is_penultimate: y.parent_id == Some(2),
            is_leaf: x.menu_type == 3,
        });
        menu_ids.push(x.id)
    }

    //不是超级管理员的时候,就要查询角色和菜单的关联
    if item.role_id != 1 {
        menu_ids.clear();
        let list = query_menu_by_role(rb, item.role_id).await?;

        for x in list {
            let m_id = x.get("menu_id").unwrap().clone();
            menu_ids.push(Some(m_id))
        }
    }

    ok_result_data(QueryRoleMenuData { menu_ids, menu_list })
}

/*
 *更新角色关联的菜单
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn update_role_menu(State(state): State<Arc<AppState>>, Json(item): Json<UpdateRoleMenuReq>) -> impl IntoResponse {
    info!("update role_menu params: {:?}", &item);
    let role_id = item.role_id;

    if role_id == 1 {
        return Err(AppError::BusinessError("不允许操作超级管理员角色"));
    }

    let rb = &state.batis;

    RoleMenu::delete_by_map(rb, value! {"role_id": &role_id}).await?;

    let mut role_menu: Vec<RoleMenu> = Vec::new();

    for id in &item.menu_ids {
        let menu_id = id.clone();
        role_menu.push(RoleMenu {
            id: None,
            create_time: Some(DateTime::now()),
            menu_id,
            role_id: role_id.clone(),
        })
    }

    RoleMenu::insert_batch(rb, &role_menu, item.menu_ids.len() as u64).await.map(|_| ok_result())?
}

/*
 *查询已分配用户角色列表
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn query_allocated_list(State(state): State<Arc<AppState>>, Json(item): Json<AllocatedListReq>) -> impl IntoResponse {
    info!("query_allocated_list params: {:?}", &item);

    let rb = &state.batis;

    let page_no = item.page_no;
    let page_size = item.page_size;
    let role_id = item.role_id;
    let mobile = item.mobile.as_deref().unwrap_or_default();
    let user_name = item.user_name.as_deref().unwrap_or_default();

    let page_no = (page_no - 1) * page_size;
    let p = select_allocated_list(rb, role_id, user_name, mobile, page_no, page_size).await?;

    let mut list: Vec<UserResp> = Vec::new();
    for x in p {
        list.push(x.into())
    }

    let total = count_allocated_list(rb, role_id, user_name, mobile).await?;
    ok_result_page(list, total)
}

/*
 *查询未分配用户角色列表
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn query_unallocated_list(State(state): State<Arc<AppState>>, Json(item): Json<UnallocatedListReq>) -> impl IntoResponse {
    info!("query_unallocated_list params: {:?}", &item);

    let rb = &state.batis;

    let page_no = item.page_no;
    let page_size = item.page_size;
    let role_id = item.role_id;
    let mobile = item.mobile.as_deref().unwrap_or_default();
    let user_name = item.user_name.as_deref().unwrap_or_default();

    let page_no = (page_no - 1) * page_size;
    let d = select_unallocated_list(rb, role_id, user_name, mobile, page_no, page_size).await?;

    let mut list: Vec<UserResp> = Vec::new();
    for x in d {
        list.push(x.into())
    }

    let total = count_unallocated_list(rb, role_id, user_name, mobile).await?;
    ok_result_page(list, total)
}

/*
 *取消授权用户
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn cancel_auth_user(State(state): State<Arc<AppState>>, Json(item): Json<CancelAuthUserReq>) -> impl IntoResponse {
    info!("cancel_auth_user params: {:?}", &item);

    let rb = &state.batis;

    delete_user_role_by_role_id_user_id(rb, item.role_id, item.user_id).await.map(|_| ok_result())?
}

/*
 *批量取消授权用户
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn batch_cancel_auth_user(State(state): State<Arc<AppState>>, Json(item): Json<CancelAuthUserAllReq>) -> impl IntoResponse {
    info!("cancel auth_user_all params: {:?}", &item);

    let rb = &state.batis;

    let update_sql = format!(
        "delete from sys_user_role where role_id = ? and user_id in ({})",
        item.user_ids.iter().map(|_| "?").collect::<Vec<&str>>().join(", ")
    );

    let mut param = vec![value!(item.role_id)];
    param.extend(item.user_ids.iter().map(|&id| value!(id)));
    rb.exec(&update_sql, param).await.map(|_| ok_result())?
}

/*
 *批量选择用户授权
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
pub async fn batch_auth_user(State(state): State<Arc<AppState>>, Json(item): Json<SelectAuthUserAllReq>) -> impl IntoResponse {
    info!("batch_auth_user params: {:?}", &item);
    let role_id = item.role_id;

    let rb = &state.batis;

    let mut user_role: Vec<UserRole> = Vec::new();

    for id in &item.user_ids {
        let user_id = id.clone();
        user_role.push(UserRole {
            id: None,
            create_time: Some(DateTime::now()),
            role_id: role_id.clone(),
            user_id,
        })
    }

    UserRole::insert_batch(rb, &user_role, item.user_ids.len() as u64).await.map(|_| ok_result())?
}
