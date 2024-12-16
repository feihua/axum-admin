// author：刘飞华
// createTime：2024/12/12 14:41:44

use serde::{Deserialize, Serialize};

/*
添加角色信息请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct AddRoleReq {
    pub role_name: String,      //名称
    pub status_id: i8,          //状态(1:正常，0:禁用)
    pub sort: i32,              //排序
    pub remark: Option<String>, //备注
}

/*
删除角色信息请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteRoleReq {
    pub ids: Vec<i64>,
}

/*
更新角色信息请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateRoleReq {
    pub id: i64,                //主键
    pub role_name: String,      //名称
    pub status_id: i8,          //状态(1:正常，0:禁用)
    pub sort: i32,              //排序
    pub remark: Option<String>, //备注
}

/*
更新角色信息状态请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateRoleStatusReq {
    pub ids: Vec<i64>,
    pub status: i8,
}

/*
查询角色信息详情请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct QueryRoleDetailReq {
    pub id: i64,
}

/*
查询角色信息详情响应参数
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct QueryRoleDetailResp {
    pub id: i64,                //主键
    pub role_name: String,      //名称
    pub status_id: i8,          //状态(1:正常，0:禁用)
    pub sort: i32,              //排序
    pub remark: Option<String>, //备注
    pub create_time: String,    //创建时间
    pub update_time: String,    //修改时间
}

impl QueryRoleDetailResp {
    pub fn new() -> QueryRoleDetailResp {
        QueryRoleDetailResp {
            id: 0,
            role_name: "".to_string(),
            status_id: 0,
            sort: 0,
            remark: None,
            create_time: "".to_string(),
            update_time: "".to_string(),
        }
    }
}

/*
查询角色信息列表请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct QueryRoleListReq {
    #[serde(rename = "current")]
    pub page_no: u64,
    #[serde(rename = "pageSize")]
    pub page_size: u64,
    pub role_name: Option<String>, //名称
    pub status_id: Option<i8>,     //状态(1:正常，0:禁用)
}

/*
查询角色信息列表响应参数
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct RoleListDataResp {
    pub id: i64,                //主键
    pub role_name: String,      //名称
    pub status_id: i8,          //状态(1:正常，0:禁用)
    pub sort: i32,              //排序
    pub remark: Option<String>, //备注
    pub create_time: String,    //创建时间
    pub update_time: String,    //修改时间
}
impl RoleListDataResp {
    pub fn new() -> Vec<RoleListDataResp> {
        Vec::new()
    }
}
/*
查询角色菜单信息参数
*/
#[derive(Debug, Deserialize)]
pub struct QueryRoleMenuReq {
    pub role_id: i64, //角色id
}

/*
角色菜单信息参数
*/
#[derive(Debug, Serialize)]
pub struct QueryRoleMenuData {
    pub menu_ids: Vec<i64>,           //菜单Ids
    pub menu_list: Vec<MenuDataList>, //菜单列表
}

/*
菜单信息参数
*/
#[derive(Debug, Serialize)]
pub struct MenuDataList {
    pub id: i64,        //主键
    pub parent_id: i64, //父ID
    pub title: String,
    pub key: String,
    pub label: String,
    #[serde(rename = "isPenultimate")]
    pub is_penultimate: bool,
}

/*
更新用户角色信息
*/
#[derive(Debug, Deserialize)]
pub struct UpdateRoleMenuReq {
    pub menu_ids: Vec<i64>,
    pub role_id: i64,
}
