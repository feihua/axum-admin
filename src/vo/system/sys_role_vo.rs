// author：刘飞华
// createTime：2024/12/12 14:41:44

use serde::{Deserialize, Serialize};

/*
添加角色信息请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddRoleReq {
    pub role_name: String,      //名称
    pub role_key: String,       //角色权限字符串
    pub data_scope: i8, //数据范围（1：全部数据权限 2：自定数据权限 3：本部门数据权限 4：本部门及以下数据权限）
    pub status: i8,     //状态(1:正常，0:禁用)
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
#[serde(rename_all = "camelCase")]
pub struct UpdateRoleReq {
    pub id: i64,                //主键
    pub role_name: String,      //名称
    pub role_key: String,       //角色权限字符串
    pub data_scope: i8, //数据范围（1：全部数据权限 2：自定数据权限 3：本部门数据权限 4：本部门及以下数据权限）
    pub status: i8,     //状态(1:正常，0:禁用)
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
#[serde(rename_all = "camelCase")]
pub struct QueryRoleDetailResp {
    pub id: i64,              //主键
    pub role_name: String,    //名称
    pub role_key: String,     //角色权限字符串
    pub data_scope: i8, //数据范围（1：全部数据权限 2：自定数据权限 3：本部门数据权限 4：本部门及以下数据权限）
    pub status: i8,     //状态(1:正常，0:禁用)
    pub remark: String, //备注
    pub del_flag: Option<i8>, //删除标志（0代表删除 1代表存在）
    pub create_time: String, //创建时间
    pub update_time: String, //修改时间
}

impl QueryRoleDetailResp {
    pub fn new() -> QueryRoleDetailResp {
        QueryRoleDetailResp {
            id: 0,
            role_name: "".to_string(),
            role_key: "".to_string(),
            data_scope: 0,
            status: 0,
            remark: "".to_string(),
            del_flag: None,
            create_time: "".to_string(),
            update_time: "".to_string(),
        }
    }
}

/*
查询角色信息列表请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryRoleListReq {
    pub page_no: u64,
    pub page_size: u64,
    pub role_name: Option<String>, //名称
    pub status_id: Option<i8>,     //状态(1:正常，0:禁用)
    pub role_key: Option<String>,  //角色权限字符串
}

/*
查询角色信息列表响应参数
*/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleListDataResp {
    pub id: i64,              //主键
    pub role_name: String,    //名称
    pub role_key: String,     //角色权限字符串
    pub data_scope: i8, //数据范围（1：全部数据权限 2：自定数据权限 3：本部门数据权限 4：本部门及以下数据权限）
    pub status: i8,     //状态(1:正常，0:禁用)
    pub remark: String, //备注
    pub del_flag: Option<i8>, //删除标志（0代表删除 1代表存在）
    pub create_time: String, //创建时间
    pub update_time: String, //修改时间
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
#[serde(rename_all = "camelCase")]
pub struct QueryRoleMenuReq {
    pub role_id: i64, //角色id
}

/*
角色菜单信息参数
*/
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryRoleMenuData {
    pub menu_ids: Vec<i64>,           //菜单Ids
    pub menu_list: Vec<MenuDataList>, //菜单列表
}

/*
菜单信息参数
*/
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MenuDataList {
    pub id: i64,        //主键
    pub parent_id: i64, //父ID
    pub title: String,
    pub key: String,
    pub label: String,
    pub is_penultimate: bool,
}

/*
更新用户角色信息
*/
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRoleMenuReq {
    pub menu_ids: Vec<i64>,
    pub role_id: i64,
}

/*
查询已分配用户角色列表
*/
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllocatedListReq {
    pub page_no: u64,
    pub page_size: u64,
    pub role_id: i64,
    pub mobile: Option<String>,
    pub user_name: Option<String>,
}

/*
查询未分配用户角色列表
*/
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnallocatedListReq {
    pub page_no: u64,
    pub page_size: u64,
    pub role_id: i64,
    pub mobile: Option<String>,
    pub user_name: Option<String>,
}

/*
取消授权用户
*/
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelAuthUserReq {
    pub user_id: i64,
    pub role_id: i64,
}

/*
批量取消授权用户
*/
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelAuthUserAllReq {
    pub user_ids: Vec<i64>,
    pub role_id: i64,
}

/*
批量选择用户授权
*/
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SelectAuthUserAllReq {
    pub user_ids: Vec<i64>,
    pub role_id: i64,
}
