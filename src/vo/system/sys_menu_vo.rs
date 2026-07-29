// author：刘飞华
// createTime：2024/12/12 14:41:44

use crate::common::result::serialize_datetime;
use crate::model::system::sys_menu_model::Menu;
use rbatis::rbdc::DateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;
/*
删除菜单信息请求参数
*/
#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct DeleteMenuReq {
    #[validate(length(min = 1, message = "id集合不能为空"))]
    pub ids: Vec<i64>,
}

/*
更新菜单信息请求参数
*/
#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MenuReq {
    pub id: Option<i64>, //主键
    #[validate(length(min = 2, max = 12, message = "菜单名称不能为空且长度为2-12"))]
    pub menu_name: String, //菜单名称
    #[validate(range(min = 1, max = 3, message = "菜单类型范围1-3"))]
    pub menu_type: i8, //菜单类型(1：目录   2：菜单   3：按钮)
    #[validate(range(min = 0, max = 1, message = "菜单状态范围0-1"))]
    pub visible: i8, //菜单状态（0:隐藏, 显示:1）
    #[validate(range(min = 0, max = 1, message = "状态（0：停用，1:正常）"))]
    pub status: i8, //状态(1:正常，0:禁用)
    pub sort: i32,       //排序
    pub parent_id: Option<i64>, //父ID
    pub menu_url: Option<String>, //路由路径
    pub api_url: Option<String>, //接口URL
    pub menu_icon: Option<String>, //菜单图标
    pub remark: Option<String>, //备注
}

/*
更新菜单信息状态请求参数
*/
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateMenuStatusReq {
    #[validate(length(min = 1, message = "id集合不能为空"))]
    pub ids: Vec<i64>,
    pub status: i8,
}

/*
查询菜单信息详情请求参数
*/
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct QueryMenuDetailReq {
    pub id: i64,
}

/*
查询菜单信息列表请求参数
*/
#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct QueryMenuListReq {
    #[validate(range(min = 1))]
    pub page_no: u64,
    #[validate(range(min = 1))]
    pub page_size: u64,
    pub menu_name: Option<String>, //菜单名称
    pub menu_type: Option<i8>,     //菜单类型(1：目录   2：菜单   3：按钮)
    pub visible: Option<i8>,       //显示状态（0:隐藏, 显示:1）
    pub status: Option<i8>,        //菜单状态(1:正常，0:禁用)
    pub parent_id: Option<i64>,    //父ID
    pub menu_url: Option<String>,  //路由路径
    pub api_url: Option<String>,   //接口URL
}

/*
查询菜单信息列表响应参数
*/
#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MenuResp {
    pub id: Option<i64>,           //主键
    pub menu_name: String,         //菜单名称
    pub menu_type: i8,             //菜单类型(1：目录   2：菜单   3：按钮)
    pub visible: i8,               //菜单状态（0:隐藏, 显示:1）
    pub status: i8,                //状态(1:正常，0:禁用)
    pub sort: i32,                 //排序
    pub parent_id: Option<i64>,    //父ID
    pub menu_url: Option<String>,  //路由路径
    pub api_url: Option<String>,   //接口URL
    pub menu_icon: Option<String>, //菜单图标
    pub remark: Option<String>,    //备注
    #[serde(serialize_with = "serialize_datetime")]
    pub create_time: Option<DateTime>, //创建时间
    #[serde(serialize_with = "serialize_datetime")]
    pub update_time: Option<DateTime>, //修改时间
}

/*
查询菜单信息列表响应参数
*/
#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MenuSimpleResp {
    pub id: Option<i64>,        //主键
    pub menu_name: String,      //菜单名称
    pub parent_id: Option<i64>, //父ID
}
impl From<Menu> for MenuSimpleResp {
    fn from(item: Menu) -> Self {
        MenuSimpleResp {
            id: item.id,               //主键
            menu_name: item.menu_name, //菜单名称
            parent_id: item.parent_id, //父ID
        }
    }
}
