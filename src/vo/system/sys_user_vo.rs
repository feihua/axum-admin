// author：刘飞华
// createTime：2024/12/12 14:41:44

use serde::{Deserialize, Serialize};

/*
添加用户信息请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct AddUserReq {
    pub mobile: String,         //手机
    pub user_name: String,      //姓名
    pub status_id: i8,          //状态(1:正常，0:禁用)
    pub sort: i32,              //排序
    pub remark: Option<String>, //备注
}

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
pub struct UpdateUserReq {
    pub id: i64,                  //主键
    pub mobile: String,           //手机
    pub user_name: String,        //姓名
    pub password: Option<String>, //密码
    pub status_id: i8,            //状态(1:正常，0:禁用)
    pub sort: i32,                //排序
    pub remark: Option<String>,   //备注
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
查询用户信息详情响应参数
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct QueryUserDetailResp {
    pub id: i64,             //主键
    pub mobile: String,      //手机
    pub user_name: String,   //姓名
    pub password: String,    //密码
    pub status_id: i8,       //状态(1:正常，0:禁用)
    pub sort: i32,           //排序
    pub remark: String,      //备注
    pub create_time: String, //创建时间
    pub update_time: String, //修改时间
}

impl QueryUserDetailResp {
    pub fn new() -> QueryUserDetailResp {
        QueryUserDetailResp {
            id: 0,
            mobile: "".to_string(),
            user_name: "".to_string(),
            password: "".to_string(),
            status_id: 0,
            sort: 0,
            remark: "".to_string(),
            create_time: "".to_string(),
            update_time: "".to_string(),
        }
    }
}

/*
查询用户信息列表请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct QueryUserListReq {
    #[serde(rename = "current")]
    pub page_no: u64,
    #[serde(rename = "pageSize")]
    pub page_size: u64,
    pub mobile: Option<String>,    //手机
    pub user_name: Option<String>, //姓名
    pub status_id: Option<i8>,     //状态(1:正常，0:禁用)
}

/*
查询用户信息列表响应参数
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct UserListDataResp {
    pub id: i64,             //主键
    pub mobile: String,      //手机
    pub user_name: String,   //姓名
    pub status_id: i8,       //状态(1:正常，0:禁用)
    pub sort: i32,           //排序
    pub remark: String,      //备注
    pub create_time: String, //创建时间
    pub update_time: String, //修改时间
}
impl UserListDataResp {
    pub fn new() -> Vec<UserListDataResp> {
        Vec::new()
    }
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
pub struct MenuList {
    pub id: i64,
    pub parent_id: i64,
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
pub struct QueryUserRoleReq {
    pub user_id: i64,
}

/*
用户关联角色响应参数
*/
#[derive(Debug, Serialize)]
pub struct QueryUserRoleResp {
    pub sys_role_list: Vec<RoleList>,
    pub user_role_ids: Vec<i64>,
}

/*
角色信息
*/
#[derive(Debug, Serialize)]
pub struct RoleList {
    pub id: i64,             //主键
    pub role_name: String,   //名称
    pub status_id: i8,       //状态(1:正常，0:禁用)
    pub sort: i32,           //排序
    pub remark: String,      //备注
    pub create_time: String, //创建时间
    pub update_time: String, //修改时间
}

/*
更新用户关联角色请求参数
*/
#[derive(Debug, Deserialize)]
pub struct UpdateUserRoleReq {
    pub user_id: i64,       //用户主键
    pub role_ids: Vec<i64>, //角色主键
}

/*
重置密码
*/
#[derive(Debug, Deserialize)]
pub struct UpdateUserPwdReq {
    pub id: i64,        //用户主键
    pub pwd: String,    //用户密码
    pub re_pwd: String, //用户密码
}
