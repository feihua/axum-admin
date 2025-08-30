// author：刘飞华
// createTime：2024/12/12 14:41:44

use crate::rbatis::rbatis_codegen::IntoSql;
use crate::vo::system::sys_menu_vo::{MenuReq, MenuResp};
use rbatis::rbdc::datetime::DateTime;
use rbatis::RBatis;
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

/*
 *菜单信息基本操作
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
rbatis::crud!(Menu {}, "sys_menu");

impl From<MenuReq> for Menu {
    fn from(item: MenuReq) -> Self {
        Menu {
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
        }
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
 *根据id查询菜单信息
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
impl_select!(Menu{select_by_id(id:&i64) -> Option => "`where id = #{id} limit 1`"}, "sys_menu");

/*
 *根据menu_name查询菜单信息
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
impl_select!(Menu{select_by_menu_name(menu_name:&str) -> Option => "`where menu_name = #{menu_name} limit 1`"}, "sys_menu");

/*
 *根据menu_url查询菜单信息
 *author：刘飞华
 *date：2025/01/04 22:24:01
 */
impl_select!(Menu{select_by_menu_url(menu_url:&str) -> Option => "`where menu_url = #{menu_url} limit 1`"}, "sys_menu");

/*
 *根据ids查询菜单信息
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
impl_select!(Menu{select_by_ids(ids:&[i64]) -> Vec => "`where status = 1 and id in ${ids.sql()} order by sort asc`"}, "sys_menu");

/*
 *查询菜单数量
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
#[sql("select count(1) from sys_menu where parent_id= ?")]
pub async fn select_count_menu_by_parent_id(rb: &RBatis, parent_id: &i64) -> rbatis::Result<i64> {
    impled!()
}

/*
 *查询菜单信息(排除按钮)
 *author：刘飞华
 *date：2025/01/04 22:24:01
 */
impl_select!(Menu{select_menu_list() -> Vec => "`where menu_type != 3 and status = 1`"}, "sys_menu");
