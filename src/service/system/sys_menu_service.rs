use crate::common::error::AppError;
use crate::model::system::sys_menu_model::{select_count_menu_by_parent_id, Menu};
use crate::model::system::sys_role_menu_model::RoleMenu;
use crate::vo::system::sys_menu_vo::*;
use crate::AppState;
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
    pub async fn add_sys_menu(state: Arc<AppState>, mut item: MenuReq) -> Result<u64, AppError> {
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
        Menu::add(rb, &Menu::from(item)).await.map(|id| Ok(id.unwrap_or_default()))?
    }

    /*
     *删除菜单信息
     *author：刘飞华
     *date：2024/12/12 14:41:44
     */
    pub async fn delete_sys_menu(state: Arc<AppState>, item: DeleteMenuReq) -> Result<u64, AppError> {
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

        Menu::delete(rb, ids).await.map(|x| Ok(x))?
    }

    /*
     *更新菜单信息
     *author：刘飞华
     *date：2024/12/12 14:41:44
     */
    pub async fn update_sys_menu(state: Arc<AppState>, item: MenuReq) -> Result<u64, AppError> {
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

        Menu::update(rb, &Menu::from(item), id.unwrap_or_default()).await.map(|x| Ok(x))?
    }

    /*
     *更新菜单信息状态
     *author：刘飞华
     *date：2024/12/12 14:41:44
     */
    pub async fn update_sys_menu_status(state: Arc<AppState>, item: UpdateMenuStatusReq) -> Result<u64, AppError> {
        let rb = &state.batis;
        Menu::update_status(rb, item.ids, item.status).await.map(|x| Ok(x))?
    }

    /*
     *查询菜单信息详情
     *author：刘飞华
     *date：2024/12/12 14:41:44
     */
    pub async fn query_sys_menu_detail(state: Arc<AppState>, item: QueryMenuDetailReq) -> Result<MenuResp, AppError> {
        let rb = &state.batis;
        Menu::select_by_id(rb, &item.id).await?.map_or_else(
            || Err(AppError::BusinessError("菜单信息不存在")),
            |x| {
                let data: MenuResp = x.into();
                Ok(data)
            },
        )
    }

    /*
     *查询菜单信息列表
     *author：刘飞华
     *date：2024/12/12 14:41:44
     */
    pub async fn query_sys_menu_list(state: Arc<AppState>) -> Result<Vec<MenuResp>, AppError> {
        let rb = &state.batis;
        Menu::select_by_map(rb, value! {}).await.map(|x| Ok(x.into_iter().map(|x| x.into()).collect::<Vec<MenuResp>>()))?
    }

    /*
     *查询菜单信息(排除按钮)
     *author：刘飞华
     *date：2024/12/12 14:41:44
     */
    pub async fn query_sys_menu_list_simple(state: Arc<AppState>) -> Result<Vec<MenuSimpleResp>, AppError> {
        let rb = &state.batis;
        Menu::select_by_map(rb, value! {"menu_type!=": 3, "status": 1})
            .await
            .map(|x| Ok(x.into_iter().map(|x| MenuSimpleResp::from(x)).collect::<Vec<MenuSimpleResp>>()))?
    }

    /*
     *查询菜单信息列表
     *author：刘飞华
     *date：2024/12/12 14:41:44
     */
    pub async fn query_sys_menu_resource_list(state: Arc<AppState>, req: QueryMenuListReq) -> Result<(Vec<MenuResp>, u64), AppError> {
        let rb = &state.batis;
        let item = &req;

        let page = &PageRequest::new(item.page_no, item.page_size);

        Menu::select_by_page(rb, page, item)
            .await
            .map(|x| Ok((x.records.into_iter().map(|x| x.into()).collect::<Vec<MenuResp>>(), x.total)))?
    }
}
