// author：刘飞华
// createTime：2024/12/12 14:41:44

use crate::common::result::serialize_datetime;
use crate::vo::system::sys_dept_vo::DeptResp;
use crate::vo::system::sys_role_vo::RoleResp;
use rbatis::rbdc::DateTime;
use serde::{Deserialize, Serialize};
/*
删除用户信息请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteUserReq {
    pub ids: Vec<i64>,
}

/*
更新用户信息请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserReq {
    pub id: Option<i64>,          //主键
    pub mobile: String,           //手机
    pub user_name: String,        //用户账号
    pub nick_name: String,        //用户昵称
    pub password: Option<String>, //用户密码
    pub email: String,            //用户邮箱
    #[serde(default = "default_avatar")]
    pub avatar: Option<String>, //头像路径
    pub status: i8,               //状态(1:正常，0:禁用)
    pub dept_id: i64,             //部门ID
    pub remark: Option<String>,   //备注
    pub post_ids: Vec<i64>,       //岗位ids
}
fn default_avatar() -> Option<String> {
    Some("https://gw.alipayobjects.com/zos/antfincdn/XAosXuNZyF/BiazfanxmamNRoxxVxka.png".to_string())
}
/*
更新用户信息状态请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserStatusReq {
    pub ids: Vec<i64>,
    pub status: i8,
}

/*
查询用户信息详情请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct QueryUserDetailReq {
    pub id: i64,
}

/*
查询用户信息列表请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryUserListReq {
    pub page_no: u64,
    pub page_size: u64,
    pub mobile: Option<String>,    //手机
    pub user_name: Option<String>, //姓名
    #[serde(default = "default_status")]
    pub status: Option<i8>, //状态(1:正常，0:禁用)
    pub dept_id: Option<i64>,      //部门ID
}
fn default_status() -> Option<i8> {
    Some(2)
}
/*
查询用户信息列表响应参数
*/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserResp {
    pub id: Option<i64>,        //主键
    pub mobile: String,         //手机
    pub user_name: String,      //用户账号
    pub nick_name: String,      //用户昵称
    pub user_type: String,      //用户类型（00系统用户）
    pub email: String,          //用户邮箱
    pub avatar: Option<String>, //头像路径
    pub status: i8,             //状态(1:正常，0:禁用)
    pub dept_id: i64,           //部门ID
    pub login_ip: String,       //最后登录IP
    #[serde(serialize_with = "serialize_datetime")]
    pub login_date: Option<DateTime>, //最后登录时间
    pub login_browser: String,  //浏览器类型
    pub login_os: String,       //操作系统
    #[serde(serialize_with = "serialize_datetime")]
    pub pwd_update_date: Option<DateTime>, //密码最后更新时间
    pub remark: Option<String>, //备注
    #[serde(serialize_with = "serialize_datetime")]
    pub create_time: Option<DateTime>, //创建时间
    #[serde(serialize_with = "serialize_datetime")]
    pub update_time: Option<DateTime>, //修改时间
    pub dept_info: Option<DeptResp>, //部门详细信息
    pub post_ids: Option<Vec<i64>>, //岗位ids
}
/*
登录请求参数
*/
#[derive(Debug, Deserialize)]
pub struct UserLoginReq {
    pub mobile: String,   //手机
    pub password: String, //密码
}

/*
查询用户菜单响应参数
*/
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryUserMenuResp {
    pub sys_menu: Vec<MenuList>,
    pub btn_menu: Vec<String>,
    pub avatar: String,
    pub name: String,
}

/*
用户菜单参数
*/
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MenuList {
    pub id: Option<i64>,
    pub parent_id: Option<i64>,
    pub name: String,
    pub path: String,
    pub api_url: String,
    pub menu_type: i8,
    pub icon: String,
}

/*
查询用户关联角色请求参数
*/
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryUserRoleReq {
    pub user_id: i64,
}

/*
用户关联角色响应参数
*/
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryUserRoleResp {
    pub sys_role_list: Vec<RoleResp>,
    pub user_role_ids: Vec<i64>,
}

/*
更新用户关联角色请求参数
*/
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserRoleReq {
    pub user_id: i64,       //用户主键
    pub role_ids: Vec<i64>, //角色主键
}

/*
重置密码
*/
#[derive(Debug, Deserialize)]
pub struct ResetUserPwdReq {
    pub id: i64,          //用户主键
    pub password: String, //用户密码
}

/*
重置密码
*/
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserPwdReq {
    pub pwd: String,    //用户密码
    pub re_pwd: String, //用户密码
}
