// author：刘飞华
// createTime：2024/12/25 10:01:11

use serde::{Deserialize, Serialize};

/*
删除系统访问记录请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteLoginLogReq {
    pub ids: Vec<i64>,
}

/*
查询系统访问记录详情请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct QueryLoginLogDetailReq {
    pub id: i64,
}

/*
查询系统访问记录详情响应参数
*/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryLoginLogDetailResp {
    pub id: i64,                //访问ID
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
    pub login_time: String,     //访问时间
}

impl QueryLoginLogDetailResp {
    pub fn new() -> QueryLoginLogDetailResp {
        QueryLoginLogDetailResp {
            id: 0,                          //访问ID
            login_name: "".to_string(),     //登录账号
            ipaddr: "".to_string(),         //登录IP地址
            login_location: "".to_string(), //登录地点
            platform: "".to_string(),       //平台信息
            browser: "".to_string(),        //浏览器类型
            version: "".to_string(),        //浏览器版本
            os: "".to_string(),             //操作系统
            arch: "".to_string(),           //体系结构信息
            engine: "".to_string(),         //渲染引擎信息
            engine_details: "".to_string(), //渲染引擎详细信息
            extra: "".to_string(),          //其他信息（可选）
            status: 0,                      //登录状态(0:失败,1:成功)
            msg: "".to_string(),            //提示消息
            login_time: "".to_string(),     //访问时间
        }
    }
}

/*
查询系统访问记录列表请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryLoginLogListReq {
    pub page_no: u64,
    pub page_size: u64,
    pub login_name: Option<String>,     //登录账号
    pub ipaddr: Option<String>,         //登录IP地址
    pub login_location: Option<String>, //登录地点
    pub browser: Option<String>,        //浏览器类型
    pub os: Option<String>,             //操作系统
    pub status: Option<i8>,             //登录状态(0:失败,1:成功)
}

/*
查询系统访问记录列表响应参数
*/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginLogListDataResp {
    pub id: i64,                //访问ID
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
    pub login_time: String,     //访问时间
}
impl LoginLogListDataResp {
    pub fn new() -> Vec<LoginLogListDataResp> {
        Vec::new()
    }
}
