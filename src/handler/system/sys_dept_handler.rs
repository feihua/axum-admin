use crate::common::error::AppError;
use crate::common::result::{ok_result, ok_result_data};
use crate::model::system::sys_dept_model::{check_dept_exist_user, select_children_dept_by_id, select_dept_count, select_normal_children_dept_by_id, Dept};
use crate::vo::system::sys_dept_vo::*;
use crate::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use axum_valid::Valid;
use rbatis::rbatis_codegen::ops::AsProxy;
use rbatis::rbdc::DateTime;
use rbs::value;
use std::sync::Arc;
use validator::Validate;
/*
 *添加部门表
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn add_sys_dept(State(state): State<Arc<AppState>>, Valid(Json(item)): Valid<Json<DeptReq>>) -> impl IntoResponse {
    log::info!("add sys_dept params: {:?}", &item);
    let rb = &state.batis;

    if Dept::select_by_dept_name(rb, &item.dept_name, item.parent_id).await?.is_some() {
        return Err(AppError::BusinessError("部门名称已存在"));
    }

    match Dept::select_by_id(rb, &item.parent_id).await? {
        None => Err(AppError::BusinessError("添加失败,上级部门不存在")),
        Some(dept) => {
            if dept.status == 0 {
                return Err(AppError::BusinessError("部门停用，不允许添加"));
            }
            let ancestors = format!("{},{}", dept.ancestors.unwrap_or_default(), &item.parent_id);
            let mut sys_dept = Dept::from(item);
            sys_dept.ancestors = Some(ancestors);
            if let Err(e) = sys_dept.validate() {
                return Err(AppError::validation_error(&e));
            }
            Dept::insert(rb, &sys_dept).await.map(|_| ok_result())?
        }
    }
}

/*
 *删除部门表
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn delete_sys_dept(State(state): State<Arc<AppState>>, Json(item): Json<DeleteDeptReq>) -> impl IntoResponse {
    log::info!("delete sys_dept params: {:?}", &item);
    let rb = &state.batis;

    if select_dept_count(rb, &item.id).await? > 0 {
        return Err(AppError::BusinessError("存在下级部门,不允许删除"));
    }

    if check_dept_exist_user(rb, &item.id).await? > 0 {
        return Err(AppError::BusinessError("部门存在用户,不允许删除"));
    }

    Dept::delete_by_map(rb, value! {"id": &item.id}).await.map(|_| ok_result())?
}

/*
 *更新部门表
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn update_sys_dept(State(state): State<Arc<AppState>>, Valid(Json(mut item)): Valid<Json<DeptReq>>) -> impl IntoResponse {
    log::info!("update sys_dept params: {:?}", &item);
    let rb = &state.batis;

    let id = item.id;
    if Some(item.parent_id) == id {
        return Err(AppError::BusinessError("上级部门不能是自己"));
    }

    let old_ancestors = match Dept::select_by_id(rb, &id.unwrap_or_default()).await? {
        None => return Err(AppError::BusinessError("部门不存在")),
        Some(dept) => dept.ancestors.unwrap_or_default(),
    };

    let ancestors = match Dept::select_by_id(rb, &item.parent_id).await? {
        None => return Err(AppError::BusinessError("上级部门不存在")),
        Some(dept) => {
            format!("{},{}", dept.ancestors.unwrap_or_default(), &item.parent_id)
        }
    };

    if let Some(dept) = Dept::select_by_dept_name(rb, &item.dept_name, item.parent_id).await? {
        if dept.id != id {
            return Err(AppError::BusinessError("部门名称已存在"));
        }
    }

    if select_normal_children_dept_by_id(rb, &id.unwrap_or_default()).await? > 0 && item.status == 0 {
        return Err(AppError::BusinessError("该部门包含未停用的子部门"));
    }

    for mut x in select_children_dept_by_id(rb, &id.unwrap_or_default()).await? {
        x.ancestors = Some(x.ancestors.unwrap_or_default().replace(old_ancestors.as_str(), ancestors.as_str()));
        Dept::update_by_map(rb, &x, value! {"id": &x.id}).await?;
    }

    if item.status == 1 && ancestors != "0" {
        let ids = ancestors.split(",").map(|s| s.i64()).collect::<Vec<i64>>();

        let update_sql = format!(
            "update sys_dept set status = ? ,update_time = ? where id in ({})",
            ids.iter().map(|_| "?").collect::<Vec<&str>>().join(", ")
        );

        let mut param = vec![value!(item.status), value!(DateTime::now())];
        param.extend(ids.iter().map(|&id| value!(id)));

        rb.exec(&update_sql, param).await?;
    }
    item.ancestors = Some(ancestors.clone());

    let mut data = Dept::from(item);
    data.update_time = Some(DateTime::now());
    if let Err(e) = data.validate() {
        return Err(AppError::validation_error(&e));
    }
    Dept::update_by_map(rb, &data, value! {"id":  &id}).await.map(|_| ok_result())?
}

/*
 *更新部门表状态
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn update_sys_dept_status(State(state): State<Arc<AppState>>, Json(item): Json<UpdateDeptStatusReq>) -> impl IntoResponse {
    log::info!("update sys_dept_status params: {:?}", &item);
    let rb = &state.batis;

    let mut ids = vec![item.id];
    if item.status == 1 {
        if let Some(x) = Dept::select_by_id(rb, &item.id).await? {
            ids.extend(&x.ancestors.unwrap_or_default().split(",").map(|s| s.i64()).collect::<Vec<i64>>())
        }
    }
    let update_sql = format!("update sys_dept set status = ? where id in ({})", ids.iter().map(|_| "?").collect::<Vec<&str>>().join(", "));

    let mut param = vec![value!(item.status)];
    param.extend(ids.iter().map(|&id| value!(id)));
    rb.exec(&update_sql, param).await.map(|_| ok_result())?
}
/*
 *查询部门表详情
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn query_sys_dept_detail(State(state): State<Arc<AppState>>, Json(item): Json<QueryDeptDetailReq>) -> impl IntoResponse {
    log::info!("query sys_dept_detail params: {:?}", &item);
    let rb = &state.batis;

    Dept::select_by_id(rb, &item.id).await?.map_or_else(
        || Err(AppError::BusinessError("部门不存在")),
        |x| {
            let data: DeptResp = x.into();
            ok_result_data(data)
        },
    )
}

/*
 *查询部门表列表
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn query_sys_dept_list(State(state): State<Arc<AppState>>, Json(item): Json<QueryDeptListReq>) -> impl IntoResponse {
    log::info!("query sys_dept_list params: {:?}", &item);
    let rb = &state.batis;

    Dept::select_page_dept_list(rb, &item)
        .await
        .map(|x| ok_result_data(x.into_iter().map(|x| x.into()).collect::<Vec<DeptResp>>()))?
}
