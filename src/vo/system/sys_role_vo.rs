// author：刘飞华
// createTime：2024/12/12 14:41:44

use crate::common::result::serialize_datetime;
use rbatis::rbdc::DateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

/*
删除角色信息请求参数
*/
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct DeleteRoleReq {
    #[validate(length(min = 1, message = "id集合不能为空"))]
    pub ids: Vec<i64>,
}

/*
更新角色信息请求参数
*/
#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RoleReq {
    pub id: Option<i64>, //主键
    #[validate(length(min = 2, max = 12, message = "名称不能为空且长度为2-12"))]
    pub role_name: String, //名称
    #[validate(length(min = 2, max = 12, message = "角色权限字符串不能为空且长度为2-12"))]
    pub role_key: String, //角色权限字符串
    #[validate(range(min = 1, max = 4, message = "数据范围（1：全部数据权限 2：自定数据权限 3：本部门数据权限 4：本部门及以下数据权限）"))]
    pub data_scope: i8, //数据范围（1：全部数据权限 2：自定数据权限 3：本部门数据权限 4：本部门及以下数据权限）
    #[validate(range(min = 0, max = 1, message = "状态（0：停用，1:正常）"))]
    pub status: i8, //状态(1:正常，0:禁用)
    pub remark: Option<String>, //备注
}

/*
更新角色信息状态请求参数
*/
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateRoleStatusReq {
    #[validate(length(min = 1, message = "id集合不能为空"))]
    pub ids: Vec<i64>,
    pub status: i8,
}

/*
查询角色信息详情请求参数
*/
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct QueryRoleDetailReq {
    pub id: i64,
}

/*
查询角色信息列表请求参数
*/
#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct QueryRoleListReq {
    #[validate(range(min = 1))]
    pub page_no: u64,
    #[validate(range(min = 1))]
    pub page_size: u64,
    pub role_name: Option<String>, //名称
    #[serde(default = "default_status")]
    pub status: Option<i8>, //状态(1:正常，0:禁用)
    pub role_key: Option<String>,  //角色权限字符串
}
fn default_status() -> Option<i8> {
    Some(2)
}
/*
查询角色信息列表响应参数
*/
#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RoleResp {
    pub id: Option<i64>,        //主键
    pub role_name: String,      //名称
    pub role_key: String,       //角色权限字符串
    pub data_scope: i8,         //数据范围（1：全部数据权限 2：自定数据权限 3：本部门数据权限 4：本部门及以下数据权限）
    pub status: i8,             //状态(1:正常，0:禁用)
    pub remark: Option<String>, //备注
    #[serde(serialize_with = "serialize_datetime")]
    pub create_time: Option<DateTime>, //创建时间
    #[serde(serialize_with = "serialize_datetime")]
    pub update_time: Option<DateTime>, //修改时间
}

/*
查询角色菜单信息参数
*/
#[derive(Debug, Deserialize, Validate)]
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
    pub menu_ids: Vec<Option<i64>>,   //菜单Ids
    pub menu_list: Vec<MenuDataList>, //菜单列表
}

/*
菜单信息参数
*/
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MenuDataList {
    pub id: Option<i64>,        //主键
    pub parent_id: Option<i64>, //父ID
    pub title: String,
    pub key: String,
    pub label: String,
    pub is_penultimate: bool,
    pub is_leaf: bool,
}

/*
更新用户角色信息
*/
#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRoleMenuReq {
    pub menu_ids: Vec<i64>,
    pub role_id: i64,
}

/*
查询已分配用户角色列表
*/
#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AllocatedListReq {
    #[validate(range(min = 1))]
    pub page_no: u64,
    #[validate(range(min = 1))]
    pub page_size: u64,
    pub role_id: i64,
    pub mobile: Option<String>,
    pub user_name: Option<String>,
}

/*
查询未分配用户角色列表
*/
#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UnallocatedListReq {
    #[validate(range(min = 1))]
    pub page_no: u64,
    #[validate(range(min = 1))]
    pub page_size: u64,
    pub role_id: i64,
    pub mobile: Option<String>,
    pub user_name: Option<String>,
}

/*
取消授权用户
*/
#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CancelAuthUserReq {
    pub user_id: i64,
    pub role_id: i64,
}

/*
批量取消授权用户
*/
#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CancelAuthUserAllReq {
    pub user_ids: Vec<i64>,
    pub role_id: i64,
}

/*
批量选择用户授权
*/
#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SelectAuthUserAllReq {
    pub user_ids: Vec<i64>,
    pub role_id: i64,
}
