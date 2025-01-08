// author：刘飞华
// createTime：2024/12/12 14:41:44

use crate::vo::system::sys_dept_vo::QueryDeptDetailResp;
use serde::{Deserialize, Serialize};
/*
添加用户信息请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddUserReq {
    pub mobile: String,         //手机
    pub user_name: String,      //用户账号
    pub nick_name: String,      //用户昵称
    pub password: String,       //用户密码
    pub email: String,          //用户邮箱
    pub avatar: Option<String>, //头像路径
    pub status: i8,             //状态(1:正常，0:禁用)
    pub dept_id: i64,           //部门ID
    pub remark: Option<String>, //备注
    pub post_ids: Vec<i64>,     //岗位ids
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
#[serde(rename_all = "camelCase")]
pub struct UpdateUserReq {
    pub id: i64,                //主键
    pub mobile: String,         //手机
    pub user_name: String,      //用户账号
    pub nick_name: String,      //用户昵称
    pub email: String,          //用户邮箱
    pub avatar: Option<String>, //头像路径
    pub status: i8,             //状态(1:正常，0:禁用)
    pub dept_id: i64,           //部门ID
    pub remark: Option<String>, //备注
    pub post_ids: Vec<i64>,     //岗位ids
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
#[serde(rename_all = "camelCase")]
pub struct QueryUserDetailResp {
    pub id: i64,                        //主键
    pub mobile: String,                 //手机
    pub user_name: String,              //用户账号
    pub nick_name: String,              //用户昵称
    pub user_type: String,              //用户类型（00系统用户）
    pub email: String,                  //用户邮箱
    pub avatar: String,                 //头像路径
    pub status: i8,                     //状态(1:正常，0:禁用)
    pub dept_id: i64,                   //部门ID
    pub login_ip: String,               //最后登录IP
    pub login_date: String,             //最后登录时间
    pub login_browser: String,          //浏览器类型
    pub login_os: String,               //操作系统
    pub pwd_update_date: String,        //密码最后更新时间
    pub remark: Option<String>,         //备注
    pub del_flag: i8,                   //删除标志（0代表删除 1代表存在）
    pub create_time: String,            //创建时间
    pub update_time: String,            //修改时间
    pub dept_info: QueryDeptDetailResp, //部门详细信息
    pub post_ids: Vec<i64>,             //岗位ids
}

impl QueryUserDetailResp {
    pub fn new() -> QueryUserDetailResp {
        QueryUserDetailResp {
            id: 0,
            mobile: "".to_string(),
            user_name: "".to_string(),
            nick_name: "".to_string(),
            user_type: "".to_string(),
            email: "".to_string(),
            avatar: "".to_string(),
            status: 0,
            dept_id: 0,
            login_ip: "".to_string(),
            login_date: "".to_string(),
            login_browser: "".to_string(),
            login_os: "".to_string(),
            pwd_update_date: "".to_string(),
            remark: None,
            del_flag: 0,
            create_time: "".to_string(),
            update_time: "".to_string(),
            dept_info: QueryDeptDetailResp {
                id: 0,
                parent_id: 0,
                ancestors: "".to_string(),
                dept_name: "".to_string(),
                sort: 0,
                leader: "".to_string(),
                phone: "".to_string(),
                email: "".to_string(),
                status: 0,
                del_flag: 0,
                create_time: "".to_string(),
                update_time: "".to_string(),
            },
            post_ids: vec![],
        }
    }
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
    pub status: Option<i8>,        //状态(1:正常，0:禁用)
    pub dept_id: Option<i64>,      //部门ID
}

/*
查询用户信息列表响应参数
*/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserListDataResp {
    pub id: i64,                 //主键
    pub mobile: String,          //手机
    pub user_name: String,       //用户账号
    pub nick_name: String,       //用户昵称
    pub user_type: String,       //用户类型（00系统用户）
    pub email: String,           //用户邮箱
    pub avatar: String,          //头像路径
    pub status: i8,              //状态(1:正常，0:禁用)
    pub dept_id: i64,            //部门ID
    pub login_ip: String,        //最后登录IP
    pub login_date: String,      //最后登录时间
    pub login_browser: String,   //浏览器类型
    pub login_os: String,        //操作系统
    pub pwd_update_date: String, //密码最后更新时间
    pub remark: Option<String>,  //备注
    pub del_flag: i8,            //删除标志（0代表删除 1代表存在）
    pub create_time: String,     //创建时间
    pub update_time: String,     //修改时间
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
    pub sys_role_list: Vec<RoleList>,
    pub user_role_ids: Vec<i64>,
}

/*
角色信息
*/
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleList {
    pub id: i64,             //主键
    pub role_name: String,   //名称
    pub role_key: String,    //角色权限字符串
    pub data_scope: i8, //数据范围（1：全部数据权限 2：自定数据权限 3：本部门数据权限 4：本部门及以下数据权限）
    pub status: i8,     //状态(1:正常，0:禁用)
    pub remark: String, //备注
    pub del_flag: i8,   //删除标志（0代表删除 1代表存在）
    pub create_time: String, //创建时间
    pub update_time: String, //修改时间
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
