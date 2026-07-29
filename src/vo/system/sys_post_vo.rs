// author：刘飞华
// createTime：2024/12/25 10:01:11

use crate::common::result::serialize_datetime;
use rbatis::rbdc::DateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

/*
删除岗位信息表请求参数
*/
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct DeletePostReq {
    #[validate(length(min = 1, message = "id集合不能为空"))]
    pub ids: Vec<i64>,
}

/*
更新岗位信息表请求参数
*/
#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PostReq {
    pub id: Option<i64>, //岗位id
    #[validate(length(min = 2, max = 12, message = "岗位编码称不能为空且长度为2-12"))]
    pub post_code: String, //岗位编码
    #[validate(length(min = 2, max = 12, message = "岗位名称不能为空且长度为2-12"))]
    pub post_name: String, //岗位名称
    #[validate(range(min = 1, message = "排序不能小于1"))]
    pub sort: i32, //显示顺序
    #[validate(range(min = 0, max = 1, message = "状态（0：停用，1:正常）"))]
    pub status: i8, //状态（0：停用，1:正常）
    pub remark: Option<String>, //备注
}

/*
更新岗位信息表状态请求参数
*/
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdatePostStatusReq {
    #[validate(length(min = 1, message = "id集合不能为空"))]
    pub ids: Vec<i64>,
    pub status: i8,
}

/*
查询岗位信息表详情请求参数
*/
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct QueryPostDetailReq {
    pub id: i64,
}

/*
查询岗位信息表列表请求参数
*/
#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct QueryPostListReq {
    #[validate(range(min = 1))]
    pub page_no: u64,
    #[validate(range(min = 1))]
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
#[derive(Debug, Serialize, Deserialize, Validate)]
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
