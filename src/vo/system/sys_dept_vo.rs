// author：刘飞华
// createTime：2024/12/25 10:01:11

use std::sync::LazyLock;
use crate::common::result::serialize_datetime;
use rbatis::rbdc::DateTime;
use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::Validate;
/*
删除部门表请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteDeptReq {
    pub id: i64,
}

static PHONE_NUMBER: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\d{11}$").unwrap()
});

/*
更新部门表请求参数
*/
#[derive(Debug, Serialize, Deserialize,Validate)]
#[serde(rename_all = "camelCase")]
pub struct DeptReq {
    pub id: Option<i64>,           //部门id
    pub parent_id: i64,            //父部门id
    #[validate(length(min = 2, max = 30, message = "部门名称不能为空且长度为2-30"))]
    pub dept_name: String,         //部门名称
    pub ancestors: Option<String>, //祖级列表
    pub sort: i32,                 //显示顺序
    #[validate(length(min = 2, max = 20, message = "负责人必填且长度为2-20"))]
    pub leader: String,            //负责人
    #[validate(regex(path = *PHONE_NUMBER, message = "联系电话格式不正确，必须为11位数字"))]
    pub phone: String,             //联系电话
    #[validate(email,length(max = 50, message = "邮箱格式不正确且长度不能超过50"))]
    pub email: String,             //邮箱
    pub status: i8,                //部状态（0：停用，1:正常）
}

/*
更新部门表状态请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDeptStatusReq {
    pub ids: Vec<i64>,
    pub status: i8,
}

/*
查询部门表详情请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct QueryDeptDetailReq {
    pub id: i64,
}

/*
查询部门表列表请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryDeptListReq {
    pub dept_name: Option<String>, //部门名称
    pub leader: Option<String>,    //负责人
    pub phone: Option<String>,     //联系电话
    pub email: Option<String>,     //邮箱
    #[serde(default = "default_status")]
    pub status: Option<i8>, //部状态（0：停用，1:正常）
}
fn default_status() -> Option<i8> {
    Some(2)
}
/*
查询部门表列表响应参数
*/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeptResp {
    pub id: Option<i64>,           //部门id
    pub parent_id: i64,            //父部门id
    pub ancestors: Option<String>, //祖级列表
    pub dept_name: String,         //部门名称
    pub sort: i32,                 //显示顺序
    pub leader: String,            //负责人
    pub phone: String,             //联系电话
    pub email: String,             //邮箱
    pub status: i8,                //部状态（0：停用，1:正常）
    #[serde(serialize_with = "serialize_datetime")]
    pub create_time: Option<DateTime>, //创建时间
    #[serde(serialize_with = "serialize_datetime")]
    pub update_time: Option<DateTime>, //修改时间
}
