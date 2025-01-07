// author：刘飞华
// createTime：2024/12/12 14:41:44

use rbatis::executor::Executor;
use rbatis::rbdc::datetime::DateTime;
use rbatis::rbdc::Error;
use serde::{Deserialize, Serialize};

/*
 *用户信息
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Option<i64>,                   //主键
    pub mobile: String,                    //手机
    pub user_name: String,                 //用户账号
    pub nick_name: String,                 //用户昵称
    pub user_type: Option<String>,         //用户类型（00系统用户）
    pub email: String,                     //用户邮箱
    pub avatar: String,                    //头像路径
    pub password: String,                  //密码
    pub status: i8,                        //状态(1:正常，0:禁用)
    pub dept_id: i64,                      //部门ID
    pub login_ip: String,                  //最后登录IP
    pub login_date: Option<DateTime>,      //最后登录时间
    pub login_browser: String,             //浏览器类型
    pub login_os: String,                  //操作系统
    pub pwd_update_date: Option<DateTime>, //密码最后更新时间
    pub remark: Option<String>,            //备注
    pub del_flag: i8,                      //删除标志（0代表删除 1代表存在）
    pub create_time: Option<DateTime>,     //创建时间
    pub update_time: Option<DateTime>,     //修改时间
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
 *根据mobile查询用户信息
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
impl_select!(User{select_by_mobile(mobile:&str) -> Option => "`where mobile = #{mobile} limit 1`"},"sys_user");

/*
 *根据user_name查询用户信息
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
impl_select!(User{select_by_user_name(user_name:&str) -> Option => "`where user_name = #{user_name} limit 1`"}, "sys_user");

/*
 *根据email查询用户信息
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
impl_select!(User{select_by_email(email:&str) -> Option => "`where email = #{email} limit 1`"}, "sys_user");

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
impl_select_page!(User{select_sys_user_list(mobile:&str,user_name:&str,status:i8,dept_id:i64) =>"
      where 1=1
      if mobile != null && mobile != '':
       ` and mobile = #{mobile} `
     if user_name != null && user_name != '':
       ` and user_name = #{user_name} `
     if status != 2:
       ` and status = #{status} `
     if dept_id != 0:
       ` and (dept_id = #{dept_id} OR dept_id IN (SELECT id FROM sys_dept WHERE find_in_set(#{dept_id}, ancestors))) `
     if !sql.contains('count'):
        ` order by create_time desc `"},"sys_user");

/*
 *根据条件分页查询已配用户角色列表
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
#[py_sql(
    "`select u.* from sys_user u left join sys_user_role ur on u.id = ur.user_id where u.del_flag = 1 and ur.role_id = #{role_id} `
            if mobile != '':
                ` and u.mobile = #{mobile} `
            if user_name != '':
                ` and u.user_name = #{user_name} `
            limit #{page_no},#{page_size}` "
)]
async fn select_allocated_list(
    rb: &dyn Executor,
    role_id: i64,
    user_name: &str,
    mobile: &str,
    page_no: u64,
    page_size: u64,
) -> Result<Vec<User>, Error> {
    impled!()
}

/*
 * 描述：根据条件分页查询已配用户角色数量
 * author：刘飞华
 * date：2025/1/6 16:13
 */
#[py_sql(
    "`select count(1) from sys_user u left join sys_user_role ur on u.id = ur.user_id where u.del_flag = 1 and ur.role_id = #{role_id} `
            if mobile != '':
                ` and u.mobile = #{mobile} `
            if user_name != '':
                ` and u.user_name = #{user_name} `"
)]
async fn count_allocated_list(
    rb: &dyn Executor,
    role_id: i64,
    user_name: &str,
    mobile: &str,
) -> Result<u64, Error> {
    impled!()
}

/*
 * 描述：根据条件分页查询未分配用户角色列表
 * author：刘飞华
 * date：2025/1/6 16:17
 */
#[py_sql(
    "`select * from sys_user where id not in (select u.id as id from sys_user u left join sys_user_role ur on u.id = ur.user_id where u.del_flag = 1 and ur.role_id = #{role_id} `
            if mobile != '':
                ` and u.mobile = #{mobile} `
            if user_name != '':
                ` and u.user_name = #{user_name} `
            ) limit #{page_no},#{page_size}` "
)]
pub async fn select_unallocated_list(
    rb: &dyn Executor,
    role_id: i64,
    user_name: &str,
    mobile: &str,
    page_no: u64,
    page_size: u64,
) -> rbatis::Result<Vec<User>> {
    impled!()
}

/*
 * 描述：根据条件分页查询未分配用户角色数量
 * author：刘飞华
 * date：2025/1/6 16:17
 */
#[py_sql(
    "`select count(1) from sys_user where id not in (select u.id as id from sys_user u left join sys_user_role ur on u.id = ur.user_id where u.del_flag = 1 and ur.role_id = #{role_id} `
            if mobile != '':
                ` and u.mobile = #{mobile} `
            if user_name != '':
                ` and u.user_name = #{user_name} `
            )` "
)]
pub async fn count_unallocated_list(
    rb: &dyn Executor,
    role_id: i64,
    user_name: &str,
    mobile: &str,
) -> rbatis::Result<u64> {
    impled!()
}
