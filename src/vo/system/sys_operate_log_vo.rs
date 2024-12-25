// author：刘飞华
// createTime：2024/12/25 10:01:11

use serde::{Deserialize, Serialize};

/*
删除操作日志记录请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteOperateLogReq {
    pub ids: Vec<i64>,
}

/*
查询操作日志记录详情请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct QueryOperateLogDetailReq {
    pub id: i64,
}

/*
查询操作日志记录详情响应参数
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct QueryOperateLogDetailResp {
    pub id: i64,                  //日志主键
    pub title: String,            //模块标题
    pub business_type: i8,        //业务类型（0其它 1新增 2修改 3删除）
    pub method: String,           //方法名称
    pub request_method: String,   //请求方式
    pub operator_type: i8,        //操作类别（0其它 1后台用户 2手机端用户）
    pub operate_name: String,     //操作人员
    pub dept_name: String,        //部门名称
    pub operate_url: String,      //请求URL
    pub operate_ip: String,       //主机地址
    pub operate_location: String, //操作地点
    pub operate_param: String,    //请求参数
    pub json_result: String,      //返回参数
    pub status: i8,               //操作状态(0:异常,正常)
    pub error_msg: String,        //错误消息
    pub operate_time: String,     //操作时间
    pub cost_time: i64,           //消耗时间
}

impl QueryOperateLogDetailResp {
    pub fn new() -> QueryOperateLogDetailResp {
        QueryOperateLogDetailResp {
            id: 0,                            //日志主键
            title: "".to_string(),            //模块标题
            business_type: 0,                 //业务类型（0其它 1新增 2修改 3删除）
            method: "".to_string(),           //方法名称
            request_method: "".to_string(),   //请求方式
            operator_type: 0,                 //操作类别（0其它 1后台用户 2手机端用户）
            operate_name: "".to_string(),     //操作人员
            dept_name: "".to_string(),        //部门名称
            operate_url: "".to_string(),      //请求URL
            operate_ip: "".to_string(),       //主机地址
            operate_location: "".to_string(), //操作地点
            operate_param: "".to_string(),    //请求参数
            json_result: "".to_string(),      //返回参数
            status: 0,                        //操作状态(0:异常,正常)
            error_msg: "".to_string(),        //错误消息
            operate_time: "".to_string(),     //操作时间
            cost_time: 0,                     //消耗时间
        }
    }
}

/*
查询操作日志记录列表请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct QueryOperateLogListReq {
    #[serde(rename = "current")]
    pub page_no: u64,
    #[serde(rename = "pageSize")]
    pub page_size: u64,
    pub title: Option<String>,            //模块标题
    pub business_type: Option<i8>,        //业务类型（0其它 1新增 2修改 3删除）
    pub method: Option<String>,           //方法名称
    pub request_method: Option<String>,   //请求方式
    pub operator_type: Option<i8>,        //操作类别（0其它 1后台用户 2手机端用户）
    pub operate_name: Option<String>,     //操作人员
    pub dept_name: Option<String>,        //部门名称
    pub operate_url: Option<String>,      //请求URL
    pub operate_ip: Option<String>,       //主机地址
    pub operate_location: Option<String>, //操作地点
    pub status: Option<i8>,               //操作状态(0:异常,正常)
}

/*
查询操作日志记录列表响应参数
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct OperateLogListDataResp {
    pub id: i64,                  //日志主键
    pub title: String,            //模块标题
    pub business_type: i8,        //业务类型（0其它 1新增 2修改 3删除）
    pub method: String,           //方法名称
    pub request_method: String,   //请求方式
    pub operator_type: i8,        //操作类别（0其它 1后台用户 2手机端用户）
    pub operate_name: String,     //操作人员
    pub dept_name: String,        //部门名称
    pub operate_url: String,      //请求URL
    pub operate_ip: String,       //主机地址
    pub operate_location: String, //操作地点
    pub operate_param: String,    //请求参数
    pub json_result: String,      //返回参数
    pub status: i8,               //操作状态(0:异常,正常)
    pub error_msg: String,        //错误消息
    pub operate_time: String,     //操作时间
    pub cost_time: i64,           //消耗时间
}
impl OperateLogListDataResp {
    pub fn new() -> Vec<OperateLogListDataResp> {
        Vec::new()
    }
}
