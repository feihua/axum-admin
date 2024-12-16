// author：刘飞华
// createTime：2024/12/12 14:41:44

use rbatis::rbdc::datetime::DateTime;
use rbatis::RBatis;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/*
 *菜单角色关联表
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RoleMenu {
    pub id: Option<i64>,               //主键
    pub role_id: i64,                  //角色ID
    pub menu_id: i64,                  //菜单ID
    pub create_time: Option<DateTime>, //创建时间
}

/*
 *菜单角色关联表基本操作
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
rbatis::crud!(RoleMenu {}, "sys_role_menu");

/*
 *根据角色id查询菜单ids
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
#[sql("select menu_id from sys_role_menu where role_id = ?")]
pub async fn query_menu_by_role(
    rb: &RBatis,
    role_id: i64,
) -> rbatis::Result<Vec<HashMap<String, i64>>> {
    impled!()
}
