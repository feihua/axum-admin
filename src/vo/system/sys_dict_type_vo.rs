// author：刘飞华
// createTime：2024/12/25 10:01:11

use crate::common::result::serialize_datetime;
use rbatis::rbdc::DateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

/*
删除字典类型表请求参数
*/
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct DeleteDictTypeReq {
    #[validate(length(min = 1, message = "id集合不能为空"))]
    pub ids: Vec<i64>,
}

/*
更新字典类型表请求参数
*/
#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DictTypeReq {
    pub id: Option<i64>, //字典主键
    #[validate(length(min = 2, max = 30, message = "字典名称标签不能为空且长度为2-12"))]
    pub dict_name: String, //字典名称
    #[validate(length(min = 2, max = 30, message = "字典类型不能为空且长度为2-12"))]
    pub dict_type: String, //字典类型
    #[validate(range(min = 0, max = 1, message = "状态（0：停用，1:正常）"))]
    pub status: i8, //状态（0：停用，1:正常）
    pub remark: Option<String>, //备注
}

/*
更新字典类型表状态请求参数
*/
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateDictTypeStatusReq {
    #[validate(length(min = 1, message = "id集合不能为空"))]
    pub ids: Vec<i64>,
    pub status: i8,
}

/*
查询字典类型表详情请求参数
*/
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct QueryDictTypeDetailReq {
    pub id: i64,
}

/*
查询字典类型表列表请求参数
*/
#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct QueryDictTypeListReq {
    #[validate(range(min = 1, message = "不能小于1"))]
    pub page_no: u64,
    #[validate(range(min = 1, message = "不能小于1"))]
    pub page_size: u64,
    pub dict_name: Option<String>, //字典名称
    pub dict_type: Option<String>, //字典类型
    #[serde(default = "default_status")]
    pub status: Option<i8>, //状态（0：停用，1:正常）
}
fn default_status() -> Option<i8> {
    Some(2)
}
/*
查询字典类型表列表响应参数
*/
#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DictTypeResp {
    pub id: Option<i64>,        //字典主键
    pub dict_name: String,      //字典名称
    pub dict_type: String,      //字典类型
    pub status: i8,             //状态（0：停用，1:正常）
    pub remark: Option<String>, //备注
    #[serde(serialize_with = "serialize_datetime")]
    pub create_time: Option<DateTime>, //创建时间
    #[serde(serialize_with = "serialize_datetime")]
    pub update_time: Option<DateTime>, //修改时间
}
