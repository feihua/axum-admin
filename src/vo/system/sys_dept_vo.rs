// author：刘飞华
// createTime：2024/12/25 10:01:11

use serde::{Deserialize, Serialize};

/*
添加部门表请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddDeptReq {
    pub parent_id: i64,    //父部门id
    pub dept_name: String, //部门名称
    pub sort: i32,         //显示顺序
    pub leader: String,    //负责人
    pub phone: String,     //联系电话
    pub email: String,     //邮箱
    pub status: i8,        //部状态（0：停用，1:正常）
}

/*
删除部门表请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteDeptReq {
    pub id: i64,
}

/*
更新部门表请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateDeptReq {
    pub id: i64,           //部门id
    pub parent_id: i64,    //父部门id
    pub dept_name: String, //部门名称
    pub sort: i32,         //显示顺序
    pub leader: String,    //负责人
    pub phone: String,     //联系电话
    pub email: String,     //邮箱
    pub status: i8,        //部状态（0：停用，1:正常）
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
查询部门表详情响应参数
*/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryDeptDetailResp {
    pub id: i64,             //部门id
    pub parent_id: i64,      //父部门id
    pub ancestors: String,   //祖级列表
    pub dept_name: String,   //部门名称
    pub sort: i32,           //显示顺序
    pub leader: String,      //负责人
    pub phone: String,       //联系电话
    pub email: String,       //邮箱
    pub status: i8,          //部状态（0：停用，1:正常）
    pub del_flag: i8,        //删除标志（0代表删除 1代表存在）
    pub create_time: String, //创建时间
    pub update_time: String, //修改时间
}

impl QueryDeptDetailResp {
    pub fn new() -> QueryDeptDetailResp {
        QueryDeptDetailResp {
            id: 0,                       //部门id
            parent_id: 0,                //父部门id
            ancestors: "".to_string(),   //祖级列表
            dept_name: "".to_string(),   //部门名称
            sort: 0,                     //显示顺序
            leader: "".to_string(),      //负责人
            phone: "".to_string(),       //联系电话
            email: "".to_string(),       //邮箱
            status: 0,                   //部状态（0：停用，1:正常）
            del_flag: 0,                 //删除标志（0代表删除 1代表存在）
            create_time: "".to_string(), //创建时间
            update_time: "".to_string(), //修改时间
        }
    }
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
    pub status: Option<i8>,        //部状态（0：停用，1:正常）
}

/*
查询部门表列表响应参数
*/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeptListDataResp {
    pub id: i64,             //部门id
    pub parent_id: i64,      //父部门id
    pub ancestors: String,   //祖级列表
    pub dept_name: String,   //部门名称
    pub sort: i32,           //显示顺序
    pub leader: String,      //负责人
    pub phone: String,       //联系电话
    pub email: String,       //邮箱
    pub status: i8,          //部状态（0：停用，1:正常）
    pub del_flag: i8,        //删除标志（0代表删除 1代表存在）
    pub create_time: String, //创建时间
    pub update_time: String, //修改时间
}
impl DeptListDataResp {
    pub fn new() -> Vec<DeptListDataResp> {
        Vec::new()
    }
}
