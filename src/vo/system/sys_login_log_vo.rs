// author：刘飞华
// createTime：2024/12/25 10:01:11
use crate::common::result::serialize_datetime;
use rbatis::rbdc::DateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

/*
删除系统访问记录请求参数
*/
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct DeleteLoginLogReq {
    #[validate(length(min = 1, message = "id集合不能为空"))]
    pub ids: Vec<i64>,
}

/*
查询系统访问记录详情请求参数
*/
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct QueryLoginLogDetailReq {
    pub id: i64,
}

/*
查询系统访问记录列表请求参数
*/
#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct QueryLoginLogListReq {
    #[validate(range(min = 1, message = "不能小于1"))]
    pub page_no: u64,
    #[validate(range(min = 1, message = "不能小于1"))]
    pub page_size: u64,
    pub login_name: Option<String>,     //登录账号
    pub ipaddr: Option<String>,         //登录IP地址
    pub login_location: Option<String>, //登录地点
    pub browser: Option<String>,        //浏览器类型
    pub os: Option<String>,             //操作系统
    #[serde(default = "default_status")]
    pub status: Option<i8>, //登录状态(0:失败,1:成功)
}
fn default_status() -> Option<i8> {
    Some(2)
}
/*
查询系统访问记录列表响应参数
*/
#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct LoginLogResp {
    pub id: Option<i64>,        //访问ID
    pub login_name: String,     //登录账号
    pub ipaddr: String,         //登录IP地址
    pub login_location: String, //登录地点
    pub platform: String,       //平台信息
    pub browser: String,        //浏览器类型
    pub version: String,        //浏览器版本
    pub os: String,             //操作系统
    pub arch: String,           //体系结构信息
    pub engine: String,         //渲染引擎信息
    pub engine_details: String, //渲染引擎详细信息
    pub extra: String,          //其他信息（可选）
    pub status: i8,             //登录状态(0:失败,1:成功)
    pub msg: String,            //提示消息
    #[serde(serialize_with = "serialize_datetime")]
    pub login_time: Option<DateTime>, //访问时间
}
