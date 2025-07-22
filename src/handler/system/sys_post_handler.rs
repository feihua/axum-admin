use crate::common::error::AppError;
use crate::common::result::{ok_result, ok_result_data, ok_result_page};
use crate::model::system::sys_post_model::Post;
use crate::model::system::sys_user_post_model::count_user_post_by_id;
use crate::utils::time_util::time_to_string;
use crate::vo::system::sys_post_vo::*;
use crate::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use rbatis::plugin::page::PageRequest;
use rbs::value;
use std::sync::Arc;
/*
 *添加岗位信息表
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn add_sys_post(State(state): State<Arc<AppState>>, Json(item): Json<AddPostReq>) -> impl IntoResponse {
    log::info!("add sys_post params: {:?}", &item);
    let rb = &state.batis;

    if Post::select_by_name(rb, &item.post_name).await?.is_some() {
        return Err(AppError::BusinessError("岗位名称已存在"));
    }

    if Post::select_by_code(rb, &item.post_code).await?.is_some() {
        return Err(AppError::BusinessError("岗位编码已存在"));
    }

    let sys_post = Post {
        id: None,                                //岗位id
        post_code: item.post_code,               //岗位编码
        post_name: item.post_name,               //岗位名称
        sort: item.sort,                         //显示顺序
        status: item.status,                     //部状态（0：停用，1:正常）
        remark: item.remark.unwrap_or_default(), //备注
        create_time: None,                       //创建时间
        update_time: None,                       //更新时间
    };

    Post::insert(rb, &sys_post).await?;

    ok_result()
}

/*
 *删除岗位信息表
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn delete_sys_post(State(state): State<Arc<AppState>>, Json(item): Json<DeletePostReq>) -> impl IntoResponse {
    log::info!("delete sys_post params: {:?}", &item);
    let rb = &state.batis;

    let ids = item.ids.clone();
    for id in ids {
        let post_by_id = Post::select_by_id(rb, &id).await?;
        let _ = match post_by_id {
            None => {
                return Err(AppError::BusinessError("岗位不存在,不能删除"));
            }
            Some(p) => p,
        };

        if count_user_post_by_id(rb, id).await? > 0 {
            return Err(AppError::BusinessError("已分配,不能删除"));
        }
    }

    Post::delete_by_map(rb, value! {"id": &item.ids}).await?;

    ok_result()
}

/*
 *更新岗位信息表
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn update_sys_post(State(state): State<Arc<AppState>>, Json(item): Json<UpdatePostReq>) -> impl IntoResponse {
    log::info!("update sys_post params: {:?}", &item);
    let rb = &state.batis;

    if Post::select_by_id(rb, &item.id).await?.is_none() {
        return Err(AppError::BusinessError("岗位不存在"));
    }

    if let Some(x) = Post::select_by_name(rb, &item.post_name).await? {
        if x.id.unwrap_or_default() != item.id {
            return Err(AppError::BusinessError("岗位名称已存在"));
        }
    }

    if let Some(x) = Post::select_by_code(rb, &item.post_code).await? {
        if x.id.unwrap_or_default() != item.id {
            return Err(AppError::BusinessError("岗位编码已存在"));
        }
    }

    let sys_post = Post {
        id: Some(item.id),                       //岗位id
        post_code: item.post_code,               //岗位编码
        post_name: item.post_name,               //岗位名称
        sort: item.sort,                         //显示顺序
        status: item.status,                     //部状态（0：停用，1:正常）
        remark: item.remark.unwrap_or_default(), //备注
        create_time: None,                       //创建时间
        update_time: None,                       //更新时间
    };

    Post::update_by_map(rb, &sys_post, value! {"id": &item.id}).await?;

    ok_result()
}

/*
 *更新岗位信息表状态
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn update_sys_post_status(State(state): State<Arc<AppState>>, Json(item): Json<UpdatePostStatusReq>) -> impl IntoResponse {
    log::info!("update sys_post_status params: {:?}", &item);
    let rb = &state.batis;

    let update_sql = format!("update sys_post set status = ? where id in ({})", item.ids.iter().map(|_| "?").collect::<Vec<&str>>().join(", "));

    let mut param = vec![value!(item.status)];
    param.extend(item.ids.iter().map(|&id| value!(id)));
    rb.exec(&update_sql, param).await?;

    ok_result()
}

/*
 *查询岗位信息表详情
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn query_sys_post_detail(State(state): State<Arc<AppState>>, Json(item): Json<QueryPostDetailReq>) -> impl IntoResponse {
    log::info!("query sys_post_detail params: {:?}", &item);
    let rb = &state.batis;

    match Post::select_by_id(rb, &item.id).await? {
        None => Err(AppError::BusinessError("岗位不存在")),
        Some(x) => {
            let sys_post = QueryPostDetailResp {
                id: x.id.unwrap_or_default(),               //岗位id
                post_code: x.post_code,                     //岗位编码
                post_name: x.post_name,                     //岗位名称
                sort: x.sort,                               //显示顺序
                status: x.status,                           //部状态（0：停用，1:正常）
                remark: x.remark,                           //备注
                create_time: time_to_string(x.create_time), //创建时间
                update_time: time_to_string(x.update_time), //更新时间
            };

            ok_result_data(sys_post)
        }
    }
}

/*
 *查询岗位信息表列表
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
pub async fn query_sys_post_list(State(state): State<Arc<AppState>>, Json(item): Json<QueryPostListReq>) -> impl IntoResponse {
    log::info!("query sys_post_list params: {:?}", &item);
    let rb = &state.batis;

    let page = &PageRequest::new(item.page_no, item.page_size);
    let d = Post::select_post_list(rb, page, &item).await?;

    let mut list: Vec<PostListDataResp> = Vec::new();

    let total = d.total;

    for x in d.records {
        list.push(PostListDataResp {
            id: x.id.unwrap_or_default(),               //岗位id
            post_code: x.post_code,                     //岗位编码
            post_name: x.post_name,                     //岗位名称
            sort: x.sort,                               //显示顺序
            status: x.status,                           //部状态（0：停用，1:正常）
            remark: x.remark,                           //备注
            create_time: time_to_string(x.create_time), //创建时间
            update_time: time_to_string(x.update_time), //更新时间
        })
    }

    ok_result_page(list, total)
}
