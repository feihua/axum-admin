// author：刘飞华
// createTime：2024/12/12 14:41:44

use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};

/*
 *角色信息
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Role {
    pub id: Option<i64>,               //主键
    pub role_name: String,             //名称
    pub role_key: String,              //角色权限字符串
    pub data_scope: Option<String>, //数据范围（1：全部数据权限 2：自定数据权限 3：本部门数据权限 4：本部门及以下数据权限）
    pub status: i8,                 //状态(1:正常，0:禁用)
    pub sort: i32,                  //排序
    pub remark: String,             //备注
    pub del_flag: i8,               //删除标志（0代表删除 1代表存在）
    pub create_time: Option<DateTime>, //创建时间
    pub update_time: Option<DateTime>, //修改时间
}

/*
 *角色信息基本操作
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
rbatis::crud!(Role {}, "sys_role");

/*
 *根据id查询角色信息
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
impl_select!(Role{select_by_id(id:&i64) -> Option => "`where id = #{id} limit 1`"}, "sys_role");

/*
 *分页查询角色信息
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
impl_select_page!(Role{select_page() =>"
     if !sql.contains('count'):
       order by create_time desc"
},"sys_role");

/*
 *根据条件分页查询角色信息
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
impl_select_page!(Role{select_page_by_name(role_name:&str,status_id:i8) =>"
      where 1=1
     if role_name != null && role_name != '':
       ` and role_name = #{role_name} `
     if status_id != 2:
       ` and status_id = #{status_id} `
     if !sql.contains('count'):
        ` order by create_time desc `"},"sys_role");
