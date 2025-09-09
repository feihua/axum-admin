// author：刘飞华
// createTime：2024/12/12 14:41:44

use crate::vo::system::sys_role_vo::{QueryRoleListReq, RoleReq, RoleResp};
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
    pub data_scope: i8,                //数据范围（1：全部数据权限 2：自定数据权限 3：本部门数据权限 4：本部门及以下数据权限）
    pub status: i8,                    //状态(1:正常，0:禁用)
    pub remark: Option<String>,        //备注
    pub del_flag: Option<i8>,          //删除标志（0代表删除 1代表存在）
    pub create_time: Option<DateTime>, //创建时间
    pub update_time: Option<DateTime>, //修改时间
}

/*
 *角色信息基本操作
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
rbatis::crud!(Role {}, "sys_role");

impl From<RoleReq> for Role {
    fn from(item: RoleReq) -> Self {
        Role {
            id: item.id,                 //主键
            role_name: item.role_name,   //名称
            role_key: item.role_key,     //角色权限字符串
            data_scope: item.data_scope, //数据范围（1：全部数据权限 2：自定数据权限 3：本部门数据权限 4：本部门及以下数据权限）
            status: item.status,         //状态(1:正常，0:禁用)
            remark: item.remark,         //备注
            del_flag: None,              //删除标志（0代表删除 1代表存在）
            create_time: None,           //创建时间
            update_time: None,           //修改时间
        }
    }
}

impl Into<RoleResp> for Role {
    fn into(self) -> RoleResp {
        RoleResp {
            id: self.id,                   //主键
            role_name: self.role_name,     //名称
            role_key: self.role_key,       //角色权限字符串
            data_scope: self.data_scope,   //数据范围（1：全部数据权限 2：自定数据权限 3：本部门数据权限 4：本部门及以下数据权限）
            status: self.status,           //状态(1:正常，0:禁用)
            remark: self.remark,           //备注
            create_time: self.create_time, //创建时间
            update_time: self.update_time, //修改时间
        }
    }
}

/*
 *根据id查询角色信息
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
impl_select!(Role{select_by_id(id:&i64) -> Option => "`where id = #{id} limit 1`"}, "sys_role");

/*
 *根据role_name查询角色信息
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
impl_select!(Role{select_by_role_name(role_name:&str) -> Option => "`where role_name = #{role_name} limit 1`"}, "sys_role");

/*
 *根据role_key查询角色信息
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
impl_select!(Role{select_by_role_key(role_key:&str) -> Option => "`where role_key = #{role_key} limit 1`"}, "sys_role");

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
impl_select_page!(Role{select_sys_role_list(req:&QueryRoleListReq) =>"
      where 1=1
     if req.roleName != null && req.roleName != '':
       ` and role_name like concat('%', #{req.roleName}, '%') `
     if req.roleKey != null && req.roleKey != '':
       ` and role_key like concat('%', #{req.roleKey}, '%') `
     if req.status != 2:
       ` and status = #{req.status} `
     if !sql.contains('count'):
        ` order by create_time desc `"},"sys_role");
