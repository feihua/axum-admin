// Code generated by https://github.com/feihua/code_cli
// author：刘飞华
// createTime：2024/12/25 10:01:11

use rbatis::rbdc::datetime::DateTime;
use rbatis::RBatis;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/*
 *部门
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Dept {
    pub id: Option<i64>,               //部门id
    pub parent_id: i64,                //父部门id
    pub ancestors: String,             //祖级列
    pub dept_name: String,             //部门名称
    pub sort: i32,                     //显示顺序
    pub leader: String,                //负责人
    pub phone: String,                 //联系电话
    pub email: String,                 //邮箱
    pub status: i8,                    //部门状态（0：停用，1:正常）
    pub del_flag: i8,                  //删除标志（0代删除 1代存在）
    pub create_time: Option<DateTime>, //创建时间
    pub update_time: Option<DateTime>, //修改时间
}

/*
 *部门基本操作
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
rbatis::crud!(Dept {}, "sys_dept");

/*
 *根据id查询部门
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
impl_select!(Dept{select_by_id(id:&i64) -> Option => "`where id = #{id} limit 1`"}, "sys_dept");

/*
 *根据部门名称查询部门
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
impl_select!(Dept{select_by_dept_name(dept_name:&str) -> Option => "`where dept_name = #{dept_name} limit 1`"}, "sys_dept");

/*
 *分页查询部门
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
impl_select_page!(Dept{select_page() =>"
     if !sql.contains('count'):
       order by create_time desc"
},"sys_dept");

/*
 *根据条件分页查询部门
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
impl_select_page!(Dept{select_page_by_name(name:&str) =>"
     if name != null && name != '':
       where real_name != #{name}
     if name == '':
       where real_name != ''"
},"sys_dept");

/*
 *根据部门id查询是否有下级部门
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
#[sql("select count(*) from sys_dept where status = 1 and del_flag = '1' and find_in_set(?}, ancestors)")]
pub async fn select_normal_children_dept_by_id(rb: &RBatis, id: &i64) -> rbatis::Result<i64> {
    impled!()
}

/*
 *根据父部门id查询下级部门数量
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
#[sql("select count(1) from sys_dept where del_flag = '1' and parent_id = ?")]
pub async fn select_dept_count(rb: &RBatis, id: &i64) -> rbatis::Result<i64> {
    impled!()
}

/*
 *查询部门是否存在用户
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
#[sql("select count(1) from sys_user where dept_id = ? and del_flag = '1'")]
pub async fn check_dept_exist_user(rb: &RBatis, id: &i64) -> rbatis::Result<i64> {
    impled!()
}
