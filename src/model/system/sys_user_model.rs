// author：刘飞华
// createTime：2024/12/12 14:41:44

use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};

/*
 *用户信息
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Option<i64>,               //主键
    pub mobile: String,                //手机
    pub user_name: String,             //姓名
    pub password: String,              //密码
    pub status_id: i8,                 //状态(1:正常，0:禁用)
    pub sort: i32,                     //排序
    pub remark: Option<String>,        //备注
    pub create_time: Option<DateTime>, //创建时间
    pub update_time: Option<DateTime>, //修改时间
}

/*
 *用户信息基本操作
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
rbatis::crud!(User {}, "sys_user");

/*
 *根据id查询用户信息
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
impl_select!(User{select_by_id(id:i64) -> Option => "`where id = #{id} limit 1`"}, "sys_user");

/*
 *分页查询用户信息
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
impl_select_page!(User{select_page() =>"
     if !sql.contains('count'):
       order by create_time desc"
},"sys_user");

/*
 *根据条件分页查询用户信息
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
impl_select_page!(User{select_page_by_name(mobile:&str,user_name:&str,status_id:i8) =>"
      where 1=1
     if mobile != null && mobile != '':
       ` and mobile = #{mobile} `
    if user_name != null && user_name != '':
       ` and user_name = #{user_name} `
     if status_id != null && status_id != '':
       ` and status_id = #{status_id} `
     if !sql.contains('count'):
        ` order by create_time desc `"},"sys_user");

impl_select!(User{select_by_mobile(mobile:&str) -> Option => "`where mobile = #{mobile} limit 1`"},"sys_user");
