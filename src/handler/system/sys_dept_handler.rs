use crate::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use rbatis::plugin::page::PageRequest;
use rbs::to_value;
use std::sync::Arc;

use crate::common::result::BaseResponse;
use crate::model::system::sys_dept_model::{
    check_dept_exist_user, select_dept_count, select_normal_children_dept_by_id, Dept,
};
use crate::utils::time_util::time_to_string;
use crate::vo::system::sys_dept_vo::*;

/*
 *添加部门表
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn add_sys_dept(
    State(state): State<Arc<AppState>>,
    Json(item): Json<AddDeptReq>,
) -> impl IntoResponse {
    log::info!("add sys_dept params: {:?}", &item);
    let rb = &state.batis;

    let res = Dept::select_by_dept_name(rb, &item.dept_name).await;
    match res {
        Ok(r) => {
            if r.is_some() {
                return BaseResponse::<String>::err_result_msg("部门名称已存在".to_string());
            }
        }
        Err(err) => return BaseResponse::<String>::err_result_msg(err.to_string()),
    }

    let sys_dept = Dept {
        id: None,                                      //部门id
        parent_id: item.parent_id,                     //父部门id
        ancestors: item.ancestors.unwrap_or_default(), //祖级列表
        dept_name: item.dept_name,                     //部门名称
        sort: item.sort,                               //显示顺序
        leader: item.leader,                           //负责人
        phone: item.phone,                             //联系电话
        email: item.email,                             //邮箱
        status: item.status,                           //部状态（0：停用，1:正常）
        del_flag: Some(1),                             //删除标志（0代表删除 1代表存在）
        create_time: None,                             //创建时间
        update_time: None,                             //修改时间
    };

    let result = Dept::insert(rb, &sys_dept).await;

    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

/*
 *删除部门表
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn delete_sys_dept(
    State(state): State<Arc<AppState>>,
    Json(item): Json<DeleteDeptReq>,
) -> impl IntoResponse {
    log::info!("delete sys_dept params: {:?}", &item);
    let rb = &state.batis;

    let res = select_dept_count(rb, &item.id).await.unwrap_or_default();
    if res > 0 {
        return BaseResponse::<String>::err_result_msg("存在下级部门,不允许删除".to_string());
    }

    let res1 = check_dept_exist_user(rb, &item.id)
        .await
        .unwrap_or_default();
    if res1 > 0 {
        return BaseResponse::<String>::err_result_msg("部门存在用户,不允许删除".to_string());
    }

    let result = Dept::delete_by_column(rb, "id", &item.id).await;

    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

/*
 *更新部门表
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn update_sys_dept(
    State(state): State<Arc<AppState>>,
    Json(item): Json<UpdateDeptReq>,
) -> impl IntoResponse {
    log::info!("update sys_dept params: {:?}", &item);
    let rb = &state.batis;

    if item.parent_id == item.id {
        return BaseResponse::<String>::err_result_msg("上级部门不能是自己".to_string());
    }

    let res = Dept::select_by_dept_name(rb, &item.dept_name).await;
    match res {
        Ok(r) => {
            if r.is_some() && r.unwrap().id.unwrap_or_default() != item.id {
                return BaseResponse::<String>::err_result_msg("部门名称已存在".to_string());
            }
        }
        Err(err) => return BaseResponse::<String>::err_result_msg(err.to_string()),
    }

    let count = select_normal_children_dept_by_id(rb, &item.id)
        .await
        .unwrap_or_default();
    if count > 0 && item.status == 0 {
        return BaseResponse::<String>::err_result_msg("该部门包含未停用的子部门".to_string());
    }

    let sys_dept = Dept {
        id: Some(item.id),         //部门id
        parent_id: item.parent_id, //父部门id
        ancestors: item.ancestors, //祖级列表
        dept_name: item.dept_name, //部门名称
        sort: item.sort,           //显示顺序
        leader: item.leader,       //负责人
        phone: item.phone,         //联系电话
        email: item.email,         //邮箱
        status: item.status,       //部状态（0：停用，1:正常）
        del_flag: None,            //删除标志（0代表删除 1代表存在）
        create_time: None,         //创建时间
        update_time: None,         //修改时间
    };

    let result = Dept::update_by_column(rb, &sys_dept, "id").await;

    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

/*
 *更新部门表状态
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn update_sys_dept_status(
    State(state): State<Arc<AppState>>,
    Json(item): Json<UpdateDeptStatusReq>,
) -> impl IntoResponse {
    log::info!("update sys_dept_status params: {:?}", &item);
    let rb = &state.batis;

    let update_sql = format!(
        "update sys_dept set status = ? where id in ({})",
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
 *查询部门表详情
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn query_sys_dept_detail(
    State(state): State<Arc<AppState>>,
    Json(item): Json<QueryDeptDetailReq>,
) -> impl IntoResponse {
    log::info!("query sys_dept_detail params: {:?}", &item);
    let rb = &state.batis;

    let result = Dept::select_by_id(rb, &item.id).await;

    match result {
        Ok(d) => {
            if d.is_none() {
                return BaseResponse::<QueryDeptDetailResp>::err_result_data(
                    QueryDeptDetailResp::new(),
                    "部门不存在".to_string(),
                );
            }
            let x = d.unwrap();

            let sys_dept = QueryDeptDetailResp {
                id: x.id.unwrap_or_default(),               //部门id
                parent_id: x.parent_id,                     //父部门id
                ancestors: x.ancestors,                     //祖级列表
                dept_name: x.dept_name,                     //部门名称
                sort: x.sort,                               //显示顺序
                leader: x.leader,                           //负责人
                phone: x.phone,                             //联系电话
                email: x.email,                             //邮箱
                status: x.status,                           //部状态（0：停用，1:正常）
                del_flag: x.del_flag.unwrap_or_default(),   //删除标志（0代表删除 1代表存在）
                create_time: time_to_string(x.create_time), //创建时间
                update_time: time_to_string(x.update_time), //修改时间
            };

            BaseResponse::<QueryDeptDetailResp>::ok_result_data(sys_dept)
        }
        Err(err) => BaseResponse::<QueryDeptDetailResp>::err_result_data(
            QueryDeptDetailResp::new(),
            err.to_string(),
        ),
    }
}

/*
 *查询部门表列表
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn query_sys_dept_list(
    State(state): State<Arc<AppState>>,
    Json(item): Json<QueryDeptListReq>,
) -> impl IntoResponse {
    log::info!("query sys_dept_list params: {:?}", &item);
    let rb = &state.batis;

    let dept_name = item.dept_name.as_deref().unwrap_or_default(); //部门名称
    let status = item.status.unwrap_or(2); //部状态（0：停用，1:正常）

    let page = &PageRequest::new(item.page_no, item.page_size);
    let result = Dept::select_page_dept_list(rb, page, dept_name, status).await;

    let mut sys_dept_list_data: Vec<DeptListDataResp> = Vec::new();
    match result {
        Ok(d) => {
            let total = d.total;

            for x in d.records {
                sys_dept_list_data.push(DeptListDataResp {
                    id: x.id.unwrap_or_default(),               //部门id
                    parent_id: x.parent_id,                     //父部门id
                    ancestors: x.ancestors,                     //祖级列表
                    dept_name: x.dept_name,                     //部门名称
                    sort: x.sort,                               //显示顺序
                    leader: x.leader,                           //负责人
                    phone: x.phone,                             //联系电话
                    email: x.email,                             //邮箱
                    status: x.status,                           //部状态（0：停用，1:正常）
                    del_flag: x.del_flag.unwrap_or_default(),   //删除标志（0代表删除 1代表存在）
                    create_time: time_to_string(x.create_time), //创建时间
                    update_time: time_to_string(x.update_time), //修改时间
                })
            }

            BaseResponse::ok_result_page(sys_dept_list_data, total)
        }
        Err(err) => BaseResponse::err_result_page(DeptListDataResp::new(), err.to_string()),
    }
}
