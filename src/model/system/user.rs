use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SysUser {
    pub id: Option<i32>,
    pub create_time: Option<DateTime>,
    pub update_time: Option<DateTime>,
    pub status_id: i32,
    pub sort: i32,
    pub mobile: String,
    pub user_name: String,
    pub remark: Option<String>,
    pub password: String,

}

rbatis::crud!(SysUser {});
impl_select_page!(SysUser{select_page() =>"
     if !sql.contains('count'):
       order by create_time desc"});

impl_select_page!(SysUser{select_page_by_name(mobile:&str,status_id:&str) =>"
      where 1=1
     if mobile != null && mobile != '':
       ` and mobile = #{mobile} `
     if status_id != null && status_id != '':
       ` and status_id = #{status_id} `
     if !sql.contains('count'):
        ` order by create_time desc `"});

impl_select!(SysUser{select_by_id(id:i32) -> Option => "`where id = #{id} limit 1`"});

impl_select!(SysUser{select_by_mobile(mobile:&str) -> Option => "`where mobile = #{mobile} limit 1`"});
