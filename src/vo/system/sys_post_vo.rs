// author：刘飞华
// createTime：2024/12/25 10:01:11

use serde::{Deserialize, Serialize};

/*
添加岗位信息表请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddPostReq {
    pub post_code: String,      //岗位编码
    pub post_name: String,      //岗位名称
    pub sort: i32,              //显示顺序
    pub status: i8,             //部状态（0：停用，1:正常）
    pub remark: Option<String>, //备注
}

/*
删除岗位信息表请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct DeletePostReq {
    pub ids: Vec<i64>,
}

/*
更新岗位信息表请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePostReq {
    pub id: i64,                //岗位id
    pub post_code: String,      //岗位编码
    pub post_name: String,      //岗位名称
    pub sort: i32,              //显示顺序
    pub status: i8,             //部状态（0：停用，1:正常）
    pub remark: Option<String>, //备注
}

/*
更新岗位信息表状态请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePostStatusReq {
    pub ids: Vec<i64>,
    pub status: i8,
}

/*
查询岗位信息表详情请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct QueryPostDetailReq {
    pub id: i64,
}

/*
查询岗位信息表详情响应参数
*/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryPostDetailResp {
    pub id: i64,             //岗位id
    pub post_code: String,   //岗位编码
    pub post_name: String,   //岗位名称
    pub sort: i32,           //显示顺序
    pub status: i8,          //部状态（0：停用，1:正常）
    pub remark: String,      //备注
    pub create_time: String, //创建时间
    pub update_time: String, //更新时间
}

impl QueryPostDetailResp {
    pub fn new() -> QueryPostDetailResp {
        QueryPostDetailResp {
            id: 0,                       //岗位id
            post_code: "".to_string(),   //岗位编码
            post_name: "".to_string(),   //岗位名称
            sort: 0,                     //显示顺序
            status: 0,                   //部状态（0：停用，1:正常）
            remark: "".to_string(),      //备注
            create_time: "".to_string(), //创建时间
            update_time: "".to_string(), //更新时间
        }
    }
}

/*
查询岗位信息表列表请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryPostListReq {
    pub page_no: u64,
    pub page_size: u64,
    pub post_code: Option<String>, //岗位编码
    pub post_name: Option<String>, //岗位名称
    pub status: Option<i8>,        //部状态（0：停用，1:正常）
}

/*
查询岗位信息表列表响应参数
*/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostListDataResp {
    pub id: i64,             //岗位id
    pub post_code: String,   //岗位编码
    pub post_name: String,   //岗位名称
    pub sort: i32,           //显示顺序
    pub status: i8,          //部状态（0：停用，1:正常）
    pub remark: String,      //备注
    pub create_time: String, //创建时间
    pub update_time: String, //更新时间
}
impl PostListDataResp {
    pub fn new() -> Vec<PostListDataResp> {
        Vec::new()
    }
}
