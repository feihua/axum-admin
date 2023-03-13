use std::collections::HashMap;
use rbatis::rbdc::datetime::FastDateTime;
use serde::{Deserialize, Serialize};
use rbatis::rbatis::Rbatis;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SysUser {
    pub  id: Option<i32>,
    pub  gmt_create: Option<FastDateTime>,
    pub  gmt_modified: Option<FastDateTime>,
    pub  status_id: Option<i32>,
    pub  sort: Option<i32>,
    pub  user_no: Option<usize>,
    pub  mobile: Option<String>,
    pub  real_name: Option<String>,
    pub  remark: Option<String>,
    pub  password: Option<String>,

}

rbatis::crud!(SysUser {});
impl_select_page!(SysUser{select_page() =>"
     if !sql.contains('count'):
       order by gmt_create desc"});

impl_select_page!(SysUser{select_page_by_name(mobile:&str,status_id:&str) =>"
      where 1=1
     if mobile != null && mobile != '':
       ` and mobile = #{mobile} `
     if status_id != null && status_id != '':
       ` and status_id = #{status_id} `
     if !sql.contains('count'):
        ` order by gmt_create desc `"});

impl_select!(SysUser{select_by_id(id:&i32) -> Option => "`where id = #{id} limit 1`"});

// user_role
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SysRoleUser {
    pub  id: Option<i32>,
    pub  gmt_create: Option<FastDateTime>,
    pub  gmt_modified: Option<FastDateTime>,
    pub  status_id: Option<i32>,
    pub  sort: Option<i32>,
    pub  role_id: Option<i32>,
    pub  user_id: Option<i32>,

}

rbatis::crud!(SysRoleUser {});
impl_select_page!(SysRoleUser{select_page() =>"
     if !sql.contains('count'):
       order by gmt_create desc"});

impl_select_page!(SysRoleUser{select_page_by_name(name:&str) =>"
     if name != null && name != '':
       where real_name != #{name}
     if name == '':
       where real_name != ''"});

// role
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SysRole {
    pub  id: Option<i32>,
    pub  gmt_create: Option<FastDateTime>,
    pub  gmt_modified: Option<FastDateTime>,
    pub  status_id: Option<i32>,
    pub  sort: Option<i32>,
    pub  role_name: Option<String>,
    pub  remark: Option<String>,

}

rbatis::crud!(SysRole {});
impl_select_page!(SysRole{select_page() =>"
     if !sql.contains('count'):
       order by gmt_create desc"});

impl_select_page!(SysRole{select_page_by_name(role_name:&str,status_id:&str) =>"
      where 1=1
     if role_name != null && role_name != '':
       ` and role_name = #{role_name} `
     if status_id != null && status_id != '':
       ` and status_id = #{status_id} `
     if !sql.contains('count'):
        ` order by gmt_create desc `"});

//role_menu
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SysMenuRole {
    pub  id: Option<i32>,
    pub  gmt_create: Option<FastDateTime>,
    pub  gmt_modified: Option<FastDateTime>,
    pub  status_id: Option<i32>,
    pub  sort: Option<i32>,
    pub  menu_id: Option<i32>,
    pub  role_id: Option<i32>,

}

rbatis::crud!(SysMenuRole {});
impl_select_page!(SysMenuRole{select_page() =>"
     if !sql.contains('count'):
       order by gmt_create desc"});

impl_select_page!(SysMenuRole{select_page_by_name(name:&str) =>"
     if name != null && name != '':
       where real_name != #{name}
     if name == '':
       where real_name != ''"});

#[sql("select menu_id from sys_menu_role where role_id = ?")]
pub async fn query_menu_by_role(rb: &Rbatis, role_id: i32) -> rbatis::Result<Vec<HashMap<String, i32>>> {
    impled!()
}

// menu
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SysMenu {
    pub  id: Option<i32>,
    pub  gmt_create: Option<FastDateTime>,
    pub  gmt_modified: Option<FastDateTime>,
    pub  status_id: Option<i32>,
    pub  sort: Option<i32>,
    pub  parent_id: Option<i32>,
    pub  menu_name: Option<String>,
    pub  menu_url: Option<String>,
    pub  api_url: Option<String>,
    pub  menu_icon: Option<String>,
    pub  remark: Option<String>,
    pub  menu_type: Option<i32>,

}

rbatis::crud!(SysMenu {});
impl_select_page!(SysMenu{select_page() =>"
     if !sql.contains('count'):
       order by gmt_create asc"});

impl_select_page!(SysMenu{select_page_by_name(name:&str) =>"
     if name != null && name != '':
       where real_name != #{name}
     if name == '':
       where real_name != ''"});


