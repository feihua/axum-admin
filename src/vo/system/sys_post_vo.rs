// author：刘飞华
// createTime：2024/12/25 10:01:11

use crate::common::result::serialize_datetime;
use rbatis::rbdc::DateTime;
use serde::{Deserialize, Serialize};

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
pub struct PostReq {
    pub id: Option<i64>,        //岗位id
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
查询岗位信息表列表请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryPostListReq {
    pub page_no: u64,
    pub page_size: u64,
    pub post_code: Option<String>, //岗位编码
    pub post_name: Option<String>, //岗位名称
    #[serde(default = "default_status")]
    pub status: Option<i8>, //部状态（0：停用，1:正常）
}
fn default_status() -> Option<i8> {
    Some(2)
}
/*
查询岗位信息表列表响应参数
*/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostResp {
    pub id: Option<i64>,        //岗位id
    pub post_code: String,      //岗位编码
    pub post_name: String,      //岗位名称
    pub sort: i32,              //显示顺序
    pub status: i8,             //部状态（0：停用，1:正常）
    pub remark: Option<String>, //备注
    #[serde(serialize_with = "serialize_datetime")]
    pub create_time: Option<DateTime>, //创建时间
    #[serde(serialize_with = "serialize_datetime")]
    pub update_time: Option<DateTime>, //修改时间
}
