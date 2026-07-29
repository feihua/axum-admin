// author：刘飞华
// createTime：2024/12/25 10:01:11

use crate::common::result::serialize_datetime;
use rbatis::rbdc::DateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

/*
删除字典数据表请求参数
*/
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct DeleteDictDataReq {
    #[validate(length(min = 1, message = "id集合不能为空"))]
    pub ids: Vec<i64>,
}

/*
更新字典数据表请求参数
*/
#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DictDataReq {
    pub id: Option<i64>, //字典编码
    #[validate(range(min = 1, message = "排序不能小于1"))]
    pub dict_sort: i32, //字典排序
    #[validate(length(min = 2, max = 30, message = "字典标签不能为空且长度为2-12"))]
    pub dict_label: String, //字典标签
    #[validate(length(min = 2, max = 30, message = "字典键值不能为空且长度为2-12"))]
    pub dict_value: String, //字典键值
    #[validate(length(min = 2, max = 30, message = "字典类型不能为空且长度为2-12"))]
    pub dict_type: String, //字典类型
    pub css_class: String, //样式属性（其他样式扩展）
    pub list_class: String, //表格回显样式
    pub is_default: String, //是否默认（Y是 N否）
    #[validate(range(min = 0, max = 1, message = "状态（0：停用，1:正常）"))]
    pub status: i8, //状态（0：停用，1:正常）
    pub remark: Option<String>, //备注
}

/*
更新字典数据表状态请求参数
*/
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateDictDataStatusReq {
    #[validate(length(min = 1, message = "id集合不能为空"))]
    pub ids: Vec<i64>,
    pub status: i8,
}

/*
查询字典数据表详情请求参数
*/
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct QueryDictDataDetailReq {
    pub id: i64,
}

/*
查询字典数据表列表请求参数
*/
#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct QueryDictDataListReq {
    #[validate(range(min = 1))]
    pub page_no: u64,
    #[validate(range(min = 1))]
    pub page_size: u64,
    pub dict_label: Option<String>, //字典标签
    pub dict_value: Option<String>, //字典键值
    pub dict_type: Option<String>,  //字典类型
    #[serde(default = "default_status")]
    pub status: Option<i8>, //状态（0：停用，1:正常）
}
fn default_status() -> Option<i8> {
    Some(2)
}
/*
查询字典数据表列表响应参数
*/
#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DictDataResp {
    pub id: Option<i64>,        //字典编码
    pub dict_sort: i32,         //字典排序
    pub dict_label: String,     //字典标签
    pub dict_value: String,     //字典键值
    pub dict_type: String,      //字典类型
    pub css_class: String,      //样式属性（其他样式扩展）
    pub list_class: String,     //表格回显样式
    pub is_default: String,     //是否默认（Y是 N否）
    pub status: i8,             //状态（0：停用，1:正常）
    pub remark: Option<String>, //备注
    #[serde(serialize_with = "serialize_datetime")]
    pub create_time: Option<DateTime>, //创建时间
    #[serde(serialize_with = "serialize_datetime")]
    pub update_time: Option<DateTime>, //修改时间
}
