// author：刘飞华
// createTime：2024/12/12 14:41:44

use crate::vo::system::sys_menu_vo::MenuResp;
use crate::vo::system::sys_menu_vo::{MenuReq, QueryMenuListReq};
use rbatis::rbdc::datetime::DateTime;
use rbatis::RBatis;
use rbs::value;
use serde::{Deserialize, Serialize};
/*
 *菜单信息
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Menu {
    pub id: Option<i64>,               //主键
    pub menu_name: String,             //菜单名称
    pub menu_type: i8,                 //菜单类型(1：目录   2：菜单   3：按钮)
    pub visible: i8,                   //菜单状态（0:隐藏, 显示:1）
    pub status: i8,                    //状态(1:正常，0:禁用)
    pub sort: i32,                     //排序
    pub parent_id: Option<i64>,        //父ID
    pub menu_url: Option<String>,      //路由路径
    pub api_url: Option<String>,       //接口URL
    pub menu_icon: Option<String>,     //菜单图标
    pub remark: Option<String>,        //备注
    pub create_time: Option<DateTime>, //创建时间
    pub update_time: Option<DateTime>, //修改时间
}
impl From<MenuReq> for Menu {
    fn from(item: MenuReq) -> Self {
        let mut model = Menu {
            id: item.id,               //主键
            menu_name: item.menu_name, //菜单名称
            menu_type: item.menu_type, //菜单类型(1：目录   2：菜单   3：按钮)
            visible: item.visible,     //菜单状态（0:隐藏, 显示:1）
            status: item.status,       //状态(1:正常，0:禁用)
            sort: item.sort,           //排序
            parent_id: item.parent_id, //父ID
            menu_url: item.menu_url,   //路由路径
            api_url: item.api_url,     //接口URL
            menu_icon: item.menu_icon, //菜单图标
            remark: item.remark,       //备注
            create_time: None,         //创建时间
            update_time: None,         //修改时间
        };
        if let None = item.id {
            model.create_time = Some(DateTime::now());
        } else {
            model.update_time = Some(DateTime::now());
        }
        model
    }
}

impl Into<MenuResp> for Menu {
    fn into(self) -> MenuResp {
        MenuResp {
            id: self.id,                   //主键
            menu_name: self.menu_name,     //菜单名称
            menu_type: self.menu_type,     //菜单类型(1：目录   2：菜单   3：按钮)
            visible: self.visible,         //菜单状态（0:隐藏, 显示:1）
            status: self.status,           //状态(1:正常，0:禁用)
            sort: self.sort,               //排序
            parent_id: self.parent_id,     //父ID
            menu_url: self.menu_url,       //路由路径
            api_url: self.api_url,         //接口URL
            menu_icon: self.menu_icon,     //菜单图标
            remark: self.remark,           //备注
            create_time: self.create_time, //创建时间
            update_time: self.update_time, //修改时间
        }
    }
}
/*
 *菜单信息基本操作
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
rbatis::crud!(Menu {}, "sys_menu");

/*
 *查询菜单数量
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
#[sql("select count(1) from sys_menu where parent_id= ?")]
pub async fn select_count_menu_by_parent_id(rb: &RBatis, parent_id: &i64) -> rbatis::Result<i64> {
    impled!()
}

impl Menu {
    /*
     *根据id查询菜单信息
     *author：刘飞华
     *date：2026/07/01 17:49:14
     */
    pub async fn select_by_id(rb: &RBatis, id: &i64) -> rbatis::Result<Option<Menu>> {
        Ok(Menu::select_by_map(rb, value! {"id": id}).await?.first().cloned())
    }

    /*
     *根据条件分页查询菜单信息
     *author：刘飞华
     *date：2026/07/01 17:49:14
     */
    #[html_sql(
        r#"<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.1//EN" "https://raw.githubusercontent.com/rbatis/rbatis/master/rbatis-codegen/mybatis-3-mapper.dtd">
      <select id="select_by_page">
            `select * from sys_menu`
            <where>
            <if test="req.menuName != '' && req.menuName != null">
                ` and menu_name like concat('%', #{req.menuName}, '%')`
            </if>
            <if test="req.status != 2">
                ` and status = #{req.status}`
            </if>
            </where>
      </select>"#
    )]
    pub async fn select_by_page(rb: &dyn rbatis::Executor, page_req: &rbatis::PageRequest, req: &QueryMenuListReq) -> rbatis::Result<rbatis::Page<Menu>> {
        impled!()
    }
}
