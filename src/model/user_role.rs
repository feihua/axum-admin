use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};

// user_role
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SysUserRole {
    pub id: Option<i32>,
    pub create_time: Option<DateTime>,
    pub update_time: Option<DateTime>,
    pub status_id: i32,
    pub sort: i32,
    pub role_id: i32,
    pub user_id: i32,

}

rbatis::crud!(SysUserRole {});
impl_select_page!(SysUserRole{select_page() =>"
     if !sql.contains('count'):
       order by create_time desc"});

impl_select_page!(SysUserRole{select_page_by_name(name:&str) =>"
     if name != null && name != '':
       where user_name != #{name}
     if name == '':
       where user_name != ''"});
