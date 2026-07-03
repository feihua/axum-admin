// author：刘飞华
// createTime：2024/12/12 14:41:44

use crate::vo::system::sys_role_vo::QueryRoleListReq;
use crate::vo::system::sys_role_vo::RoleReq;
use crate::vo::system::sys_role_vo::RoleResp;
use rbatis::rbdc::datetime::DateTime;
use rbatis::RBatis;
use rbs::value;
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
        let mut model = Role {
            id: item.id,                 //主键
            role_name: item.role_name,   //名称
            role_key: item.role_key,     //角色权限字符串
            data_scope: item.data_scope, //数据范围（1：全部数据权限 2：自定数据权限 3：本部门数据权限 4：本部门及以下数据权限）
            status: item.status,         //状态(1:正常，0:禁用)
            remark: item.remark,         //备注
            del_flag: None,              //删除标志（0代表删除 1代表存在）
            create_time: None,           //创建时间
            update_time: None,           //修改时间
        };
        if let None = item.id {
            model.create_time = Some(DateTime::now());
        } else {
            model.update_time = Some(DateTime::now());
        }
        model
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

impl Role {
    /*
     *根据id查询角色信息
     *author：刘飞华
     *date：2026/07/01 17:49:14
     */
    pub async fn select_by_id(rb: &RBatis, id: &i64) -> rbatis::Result<Option<Role>> {
        Ok(Role::select_by_map(rb, value! {"id": id}).await?.first().cloned())
    }

    /*
     *根据条件分页查询角色信息
     *author：刘飞华
     *date：2026/07/01 17:49:14
     */
    #[html_sql(
        r#"<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.1//EN" "https://raw.githubusercontent.com/rbatis/rbatis/master/rbatis-codegen/mybatis-3-mapper.dtd">
      <select id="select_by_page">
            `select * from sys_role`
            <where>
            <if test="req.roleName != '' && req.roleName != null">
                ` and role_name like concat('%', #{req.roleName}, '%')`
            </if>
            <if test="req.roleKey != '' && req.roleKey != null">
                ` and role_key like concat('%', #{req.roleKey}, '%')`
            </if>
            <if test="req.status != 2">
                ` and status = #{req.status}`
            </if>
            </where>
      </select>"#
    )]
    pub async fn select_by_page(rb: &dyn rbatis::Executor, page_req: &rbatis::PageRequest, req: &QueryRoleListReq) -> rbatis::Result<rbatis::Page<Role>> {
        impled!()
    }
}
