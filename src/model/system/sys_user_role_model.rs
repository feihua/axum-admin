// author：刘飞华
// createTime：2024/12/12 14:41:44

use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};

/*
 *角色用户关联表
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserRole {
    pub id: Option<i64>,               //主键
    pub user_id: i64,                  //用户ID
    pub role_id: i64,                  //角色ID
    pub status_id: i8,                 //状态(1:正常，0:禁用)
    pub sort: i32,                     //排序
    pub create_time: Option<DateTime>, //创建时间
    pub update_time: Option<DateTime>, //修改时间
}

/*
 *角色用户关联表基本操作
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
rbatis::crud!(UserRole {}, "sys_user_role");

/*
 *查询是否为超级管理员(role_id=1是预设超级管理的id)
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
impl_select!(UserRole{is_admin(user_id:&i64) => "`where user_id = #{user_id} and role_id = 1`"},"sys_user_role");
