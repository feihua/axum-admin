// author：刘飞华
// createTime：2024/12/25 10:01:11

use serde::{Deserialize, Serialize};

/*
添加通知公告表请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddNoticeReq {
    pub notice_title: String,   //公告标题
    pub notice_type: i8,        //公告类型（1:通知,2:公告）
    pub notice_content: String, //公告内容
    pub status: i8,             //公告状态（0:关闭,1:正常 ）
    pub remark: Option<String>, //备注
}

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
pub struct UpdateNoticeReq {
    pub id: i64,                //公告ID
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
查询通知公告表详情响应参数
*/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryNoticeDetailResp {
    pub id: i64,                //公告ID
    pub notice_title: String,   //公告标题
    pub notice_type: i8,        //公告类型（1:通知,2:公告）
    pub notice_content: String, //公告内容
    pub status: i8,             //公告状态（0:关闭,1:正常 ）
    pub remark: String,         //备注
    pub create_time: String,    //创建时间
    pub update_time: String,    //修改时间
}

impl QueryNoticeDetailResp {
    pub fn new() -> QueryNoticeDetailResp {
        QueryNoticeDetailResp {
            id: 0,                          //公告ID
            notice_title: "".to_string(),   //公告标题
            notice_type: 0,                 //公告类型（1:通知,2:公告）
            notice_content: "".to_string(), //公告内容
            status: 0,                      //公告状态（0:关闭,1:正常 ）
            remark: "".to_string(),         //备注
            create_time: "".to_string(),    //创建时间
            update_time: "".to_string(),    //修改时间
        }
    }
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
    pub status: Option<i8>,           //公告状态（0:关闭,1:正常 ）
}

/*
查询通知公告表列表响应参数
*/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NoticeListDataResp {
    pub id: i64,                //公告ID
    pub notice_title: String,   //公告标题
    pub notice_type: i8,        //公告类型（1:通知,2:公告）
    pub notice_content: String, //公告内容
    pub status: i8,             //公告状态（0:关闭,1:正常 ）
    pub remark: String,         //备注
    pub create_time: String,    //创建时间
    pub update_time: String,    //修改时间
}
impl NoticeListDataResp {
    pub fn new() -> Vec<NoticeListDataResp> {
        Vec::new()
    }
}
