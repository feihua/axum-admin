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
#[serde(rename_all = "camelCase")]
pub struct QueryOperateLogDetailResp {
    pub id: Option<i64>,                  //日志主键
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
    pub operate_param: Option<String>,    //请求参数
    pub json_result: Option<String>,      //返回参数
    pub status: Option<i8>,               //操作状态(0:异常,正常)
    pub error_msg: Option<String>,        //错误消息
    pub operate_time: String,             //操作时间
    pub cost_time: Option<i64>,           //消耗时间
}

impl QueryOperateLogDetailResp {
    pub fn new() -> QueryOperateLogDetailResp {
        QueryOperateLogDetailResp {
            id: None,
            title: None,
            business_type: None,
            method: None,
            request_method: None,
            operator_type: None,
            operate_name: None,
            dept_name: None,
            operate_url: None,
            operate_ip: None,
            operate_location: None,
            operate_param: None,
            json_result: None,
            status: None,
            error_msg: None,
            operate_time: "".to_string(),
            cost_time: None,
        }
    }
}

/*
查询操作日志记录列表请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryOperateLogListReq {
    pub page_no: u64,
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
#[serde(rename_all = "camelCase")]
pub struct OperateLogListDataResp {
    pub id: Option<i64>,                  //日志主键
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
    pub operate_param: Option<String>,    //请求参数
    pub json_result: Option<String>,      //返回参数
    pub status: Option<i8>,               //操作状态(0:异常,正常)
    pub error_msg: Option<String>,        //错误消息
    pub operate_time: String,             //操作时间
    pub cost_time: Option<i64>,           //消耗时间
}
impl OperateLogListDataResp {
    pub fn new() -> Vec<OperateLogListDataResp> {
        Vec::new()
    }
}
