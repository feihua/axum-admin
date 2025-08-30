// author：刘飞华
// createTime：2024/12/25 10:01:11

use crate::common::result::serialize_datetime;
use rbatis::rbdc::DateTime;
use serde::{Deserialize, Serialize};

/*
删除通知公告表请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteNoticeReq {
    pub ids: Vec<i64>,
}

/*
更新通知公告表请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NoticeReq {
    pub id: Option<i64>,        //公告ID
    pub notice_title: String,   //公告标题
    pub notice_type: i8,        //公告类型（1:通知,2:公告）
    pub notice_content: String, //公告内容
    pub status: i8,             //公告状态（0:关闭,1:正常 ）
    pub remark: Option<String>, //备注
}

/*
更新通知公告表状态请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateNoticeStatusReq {
    pub ids: Vec<i64>,
    pub status: i8,
}

/*
查询通知公告表详情请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct QueryNoticeDetailReq {
    pub id: i64,
}

/*
查询通知公告表列表请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryNoticeListReq {
    pub page_no: u64,
    pub page_size: u64,
    pub notice_title: Option<String>, //公告标题
    pub notice_type: Option<i8>,      //公告类型（1:通知,2:公告）
    #[serde(default = "default_status")]
    pub status: Option<i8>, //公告状态（0:关闭,1:正常 ）
}
fn default_status() -> Option<i8> {
    Some(2)
}
/*
查询通知公告表列表响应参数
*/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NoticeResp {
    pub id: Option<i64>,        //公告ID
    pub notice_title: String,   //公告标题
    pub notice_type: i8,        //公告类型（1:通知,2:公告）
    pub notice_content: String, //公告内容
    pub status: i8,             //公告状态（0:关闭,1:正常 ）
    pub remark: Option<String>, //备注
    #[serde(serialize_with = "serialize_datetime")]
    pub create_time: Option<DateTime>, //创建时间
    #[serde(serialize_with = "serialize_datetime")]
    pub update_time: Option<DateTime>, //修改时间
}
