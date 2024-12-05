use std::collections::HashMap;
use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};
use rbatis::rbatis::RBatis;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SysRoleMenu {
    pub id: Option<i32>,
    pub create_time: Option<DateTime>,
    pub update_time: Option<DateTime>,
    pub status_id: i32,
    pub sort: i32,
    pub menu_id: i32,
    pub role_id: i32,

}

rbatis::crud!(SysRoleMenu {});
impl_select_page!(SysRoleMenu{select_page() =>"
     if !sql.contains('count'):
       order by create_time desc"});

impl_select_page!(SysRoleMenu{select_page_by_name(name:&str) =>"
     if name != null && name != '':
       where user_name != #{name}
     if name == '':
       where user_name != ''"});

#[sql("select menu_id from sys_role_menu where role_id = ?")]
pub async fn query_menu_by_role(rb: &RBatis, role_id: i32) -> rbatis::Result<Vec<HashMap<String, i32>>> {
    impled!()
}