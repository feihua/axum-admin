use crate::common::error::AppError;
use crate::common::result::{ok_result, ok_result_data, ok_result_page};
use crate::model::system::sys_menu_model::{select_count_menu_by_parent_id, Menu};
use crate::model::system::sys_role_menu_model::RoleMenu;
use crate::vo::system::sys_menu_vo::*;
use crate::AppState;
use axum::response::IntoResponse;
use rbatis::PageRequest;
use rbs::value;
use std::sync::Arc;

pub struct MenuService;

impl MenuService {
    /*
     *添加菜单信息
     *author：刘飞华
     *date：2024/12/12 14:41:44
     */
    pub async fn add_sys_menu(state: Arc<AppState>, mut item: MenuReq) -> impl IntoResponse {
        let rb = &state.batis;
        if Menu::select_by_map(rb, value! {"menu_name": &item.menu_name}).await?.len() > 0 {
            return Err(AppError::BusinessError("菜单名称已存在"));
        }

        if let Some(url) = item.menu_url.clone() {
            if url != "".to_string() {
                if Menu::select_by_map(rb, value! {"menu_url": url}).await?.len() > 0 {
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
    pub async fn delete_sys_menu(state: Arc<AppState>, item: DeleteMenuReq) -> impl IntoResponse {
        let rb = &state.batis;
        let ids = item.ids;
        for x in &ids {
            if select_count_menu_by_parent_id(rb, x).await? > 0 {
                return Err(AppError::BusinessError("存在子菜单,不允许删除"));
            }

            if RoleMenu::select_count_menu_by_menu_id(rb, x).await? > 0 {
                return Err(AppError::BusinessError("菜单已分配,不允许删除"));
            }
        }

        Menu::delete_by_map(rb, value! {"id": ids}).await.map(|_| ok_result())?
    }

    /*
     *更新菜单信息
     *author：刘飞华
     *date：2024/12/12 14:41:44
     */
    pub async fn update_sys_menu(state: Arc<AppState>, item: MenuReq) -> impl IntoResponse {
        let rb = &state.batis;
        let id = item.id;

        if id.is_none() {
            return Err(AppError::BusinessError("主键不能为空"));
        }

        if Menu::select_by_id(rb, &id.unwrap_or_default()).await?.is_none() {
            return Err(AppError::BusinessError("菜单信息不存在"));
        }

        if Menu::select_by_map(rb, value! {"menu_name": &item.menu_name, "id !=": id}).await?.len() > 0 {
            return Err(AppError::BusinessError("菜单名称已存在"));
        }

        if let Some(url) = item.menu_url.clone() {
            if url != "".to_string() {
                if Menu::select_by_map(rb, value! {"menu_url": url, "id !=": id}).await?.len() > 0 {
                    return Err(AppError::BusinessError("路由路径已存在"));
                }
            }
        }

        Menu::update_by_map(rb, &Menu::from(item), value! {"id": id}).await.map(|_| ok_result())?
    }

    /*
     *更新菜单信息状态
     *author：刘飞华
     *date：2024/12/12 14:41:44
     */
    pub async fn update_sys_menu_status(state: Arc<AppState>, item: UpdateMenuStatusReq) -> impl IntoResponse {
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
    pub async fn query_sys_menu_detail(state: Arc<AppState>, item: QueryMenuDetailReq) -> impl IntoResponse {
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
    pub async fn query_sys_menu_list(state: Arc<AppState>) -> impl IntoResponse {
        let rb = &state.batis;
        Menu::select_by_map(rb, value! {})
            .await
            .map(|x| ok_result_data(x.into_iter().map(|x| x.into()).collect::<Vec<MenuResp>>()))?
    }

    /*
     *查询菜单信息(排除按钮)
     *author：刘飞华
     *date：2024/12/12 14:41:44
     */
    pub async fn query_sys_menu_list_simple(state: Arc<AppState>) -> impl IntoResponse {
        let rb = &state.batis;
        Menu::select_by_map(rb, value! {"menu_type!=": 3, "status": 1})
            .await
            .map(|x| ok_result_data(x.into_iter().map(|x| MenuSimpleResp::from(x)).collect::<Vec<MenuSimpleResp>>()))?
    }

    /*
     *查询菜单信息列表
     *author：刘飞华
     *date：2024/12/12 14:41:44
     */
    pub async fn query_sys_menu_resource_list(state: Arc<AppState>, req: QueryMenuListReq) -> impl IntoResponse {
        let rb = &state.batis;
        let item = &req;

        let page = &PageRequest::new(item.page_no, item.page_size);

        Menu::select_by_page(rb, page, item)
            .await
            .map(|x| ok_result_page(x.records.into_iter().map(|x| x.into()).collect::<Vec<MenuResp>>(), x.total))?
    }
}
