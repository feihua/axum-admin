use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SysRole {
    pub id: Option<i32>,
    pub create_time: Option<DateTime>,
    pub update_time: Option<DateTime>,
    pub status_id: i32,
    pub sort: i32,
    pub role_name: String,
    pub remark: Option<String>,

}

rbatis::crud!(SysRole {});
impl_select_page!(SysRole{select_page() =>"
     if !sql.contains('count'):
       order by create_time desc"});

impl_select_page!(SysRole{select_page_by_name(role_name:&str,status_id:&str) =>"
      where 1=1
     if role_name != null && role_name != '':
       ` and role_name = #{role_name} `
     if status_id != null && status_id != '':
       ` and status_id = #{status_id} `
     if !sql.contains('count'):
        ` order by create_time desc `"});
