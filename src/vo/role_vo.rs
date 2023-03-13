use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct RoleListReq {
    #[serde(rename = "current")]
    pub page_no: u64,
    #[serde(rename = "pageSize")]
    pub page_size: u64,
    pub role_name: Option<String>,
    pub status_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct RoleListResp {
    pub msg: String,
    pub code: i32,
    pub page_no: u64,
    pub page_size: u64,
    pub success: bool,
    pub total: u64,
    pub data: Option<Vec<RoleListData>>,
}

#[derive(Debug, Serialize)]
pub struct RoleListData {
    pub id: i32,
    pub sort: i32,
    pub status_id: i32,
    pub role_name: String,
    pub remark: String,
    pub create_time: String,
    pub update_time: String,
}

#[derive(Debug, Deserialize)]
pub struct RoleSaveReq {
    pub role_name: String,
    pub sort: i32,
    pub remark: String,
}

#[derive(Debug, Deserialize)]
pub struct RoleUpdateReq {
    pub id: i32,
    pub sort: i32,
    pub status_id: i32,
    pub role_name: String,
    pub remark: String,
}


#[derive(Debug, Deserialize)]
pub struct RoleDeleteReq {
    pub ids: Vec<i32>,
}


#[derive(Debug, Deserialize)]
pub struct QueryRoleMenuReq {
    pub role_id: i32,
}

#[derive(Debug, Serialize)]
pub struct QueryRoleMenuResp {
    pub msg: String,
    pub code: i32,
    pub data: QueryRoleMenuData,
}

#[derive(Debug, Serialize)]
pub struct QueryRoleMenuData {
    pub role_menus: Vec<i32>,
    pub menu_list: Vec<MenuDataList>,
}

#[derive(Debug, Serialize)]
pub struct MenuDataList {
    pub id: i32,
    pub parent_id: i32,
    pub title: String,
    pub key: String,
    pub label: String,
    #[serde(rename = "isPenultimate")]
    pub is_penultimate: bool,
}

#[derive(Debug, Deserialize)]
pub struct UpdateRoleMenuReq {
    pub menu_ids: Vec<i32>,
    pub role_id: i32,
}


