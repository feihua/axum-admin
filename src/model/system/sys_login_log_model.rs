// Code generated by https://github.com/feihua/code_cli
// author：刘飞华
// createTime：2024/12/25 10:01:11

use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};

/*
 *系统访问记录
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoginLog {
    pub id: Option<i64>,              //访问ID
    pub login_name: String,           //登录账号
    pub ipaddr: String,               //登录IP地址
    pub login_location: String,       //登录地点
    pub browser: String,              //浏览器类型
    pub os: String,                   //操作系统
    pub status: i8,                   //登录状态(0:失败,1:成功)
    pub msg: String,                  //提示消息
    pub login_time: Option<DateTime>, //访问时间
}

/*
 *系统访问记录基本操作
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
rbatis::crud!(LoginLog {}, "sys_login_log");

/*
 *根据id查询系统访问记录
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
impl_select!(LoginLog{select_by_id(id:&i64) -> Option => "`where id = #{id} limit 1`"}, "sys_login_log");

/*
 *分页查询系统访问记录
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
impl_select_page!(LoginLog{select_page() =>"
     if !sql.contains('count'):
       order by login_time desc"
},"sys_login_log");

/*
 *根据条件分页查询系统访问记录
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
impl_select_page!(LoginLog{select_page_by_name(name:&str) =>"
     if name != null && name != '':
       where real_name != #{name}
     if name == '':
       where real_name != ''"
},"sys_login_log");
