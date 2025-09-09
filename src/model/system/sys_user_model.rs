// author：刘飞华
// createTime：2024/12/12 14:41:44

use crate::vo::system::sys_user_vo::{QueryUserListReq, UserReq, UserResp};
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
    pub avatar: Option<String>,            //头像路径
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

impl From<UserReq> for User {
    fn from(item: UserReq) -> Self {
        User {
            id: None,                                    //主键
            mobile: item.mobile,                         //手机
            user_name: item.user_name,                   //用户账号
            nick_name: item.nick_name,                   //用户昵称
            user_type: Some("01".to_string()),           //用户类型（00系统用户）
            email: item.email,                           //用户邮箱
            avatar: item.avatar,                         //头像路径
            password: item.password.unwrap_or_default(), //密码
            status: item.status,                         //状态(1:正常，0:禁用)
            dept_id: item.dept_id,                       //部门ID
            login_ip: "".to_string(),                    //最后登录IP
            login_date: None,                            //最后登录时间
            login_browser: "".to_string(),               //浏览器类型
            login_os: "".to_string(),                    //操作系统
            pwd_update_date: None,                       //密码最后更新时间
            remark: item.remark,                         //备注
            del_flag: 1,                                 //删除标志（0代表删除 1代表存在）
            create_time: None,                           //创建时间
            update_time: None,                           //修改时间
        }
    }
}

impl Into<UserResp> for User {
    fn into(self) -> UserResp {
        UserResp {
            id: self.id,                                   //主键
            mobile: self.mobile,                           //手机
            user_name: self.user_name,                     //姓名
            nick_name: self.nick_name,                     //用户昵称
            user_type: self.user_type.unwrap_or_default(), //用户类型（00系统用户）
            email: self.email,                             //用户邮箱
            avatar: self.avatar,                           //头像路径
            status: self.status,                           //状态(1:正常，0:禁用)
            dept_id: self.dept_id,                         //部门ID
            login_ip: self.login_ip,                       //最后登录IP
            login_date: self.login_date,                   //最后登录时间
            login_browser: self.login_browser,             //浏览器类型
            login_os: self.login_os,                       //操作系统
            pwd_update_date: self.pwd_update_date,         //密码最后更新时间
            remark: self.remark,                           //备注
            create_time: self.create_time,                 //创建时间
            update_time: self.update_time,                 //修改时间
            dept_info: None,
            post_ids: None,
        }
    }
}
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
impl_select_page!(User{select_sys_user_list(req:&QueryUserListReq) =>"
      where 1=1
      if req.mobile != null && req.mobile != '':
       ` and mobile = #{req.mobile} `
     if req.userName != null && req.userName != '':
       ` and user_name = #{req.userName} `
     if req.status != 2:
       ` and status = #{req.status} `
     if req.deptId != 0:
       ` and (dept_id = #{req.deptId} OR dept_id IN (SELECT id FROM sys_dept WHERE find_in_set(#{req.deptId}, ancestors))) `
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
async fn select_allocated_list(rb: &dyn Executor, role_id: i64, user_name: &str, mobile: &str, page_no: u64, page_size: u64) -> Result<Vec<User>, Error> {
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
async fn count_allocated_list(rb: &dyn Executor, role_id: i64, user_name: &str, mobile: &str) -> Result<u64, Error> {
    impled!()
}

/*
 * 描述：根据条件分页查询未分配用户角色列表
 * author：刘飞华
 * date：2025/1/6 16:17
 */
#[py_sql(
    "`select u.* from sys_user u left join sys_user_role ur on u.id = ur.user_id where u.del_flag = 1 and (ur.role_id != #{role_id} or ur.role_id is null) `
            if mobile != '':
                ` and u.mobile = #{mobile} `
            if user_name != '':
                ` and u.user_name = #{user_name} `
            limit #{page_no},#{page_size}` "
)]
pub async fn select_unallocated_list(rb: &dyn Executor, role_id: i64, user_name: &str, mobile: &str, page_no: u64, page_size: u64) -> rbatis::Result<Vec<User>> {
    impled!()
}

/*
 * 描述：根据条件分页查询未分配用户角色数量
 * author：刘飞华
 * date：2025/1/6 16:17
 */
#[py_sql(
    "`select count(1) from sys_user u left join sys_user_role ur on u.id = ur.user_id where u.del_flag = 1 and (ur.role_id != #{role_id} or ur.role_id is null) `
            if mobile != '':
                ` and u.mobile = #{mobile} `
            if user_name != '':
                ` and u.user_name = #{user_name} `"
)]
pub async fn count_unallocated_list(rb: &dyn Executor, role_id: i64, user_name: &str, mobile: &str) -> rbatis::Result<u64> {
    impled!()
}
