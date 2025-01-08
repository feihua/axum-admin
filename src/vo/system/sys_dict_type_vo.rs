// author：刘飞华
// createTime：2024/12/25 10:01:11

use serde::{Deserialize, Serialize};

/*
添加字典类型表请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddDictTypeReq {
    pub dict_name: String,      //字典名称
    pub dict_type: String,      //字典类型
    pub status: i8,             //状态（0：停用，1:正常）
    pub remark: Option<String>, //备注
}

/*
删除字典类型表请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteDictTypeReq {
    pub ids: Vec<i64>,
}

/*
更新字典类型表请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateDictTypeReq {
    pub dict_id: i64,           //字典主键
    pub dict_name: String,      //字典名称
    pub dict_type: String,      //字典类型
    pub status: i8,             //状态（0：停用，1:正常）
    pub remark: Option<String>, //备注
}

/*
更新字典类型表状态请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDictTypeStatusReq {
    pub ids: Vec<i64>,
    pub status: i8,
}

/*
查询字典类型表详情请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct QueryDictTypeDetailReq {
    pub id: i64,
}

/*
查询字典类型表详情响应参数
*/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryDictTypeDetailResp {
    pub dict_id: i64,        //字典主键
    pub dict_name: String,   //字典名称
    pub dict_type: String,   //字典类型
    pub status: i8,          //状态（0：停用，1:正常）
    pub remark: String,      //备注
    pub create_time: String, //创建时间
    pub update_time: String, //修改时间
}

impl QueryDictTypeDetailResp {
    pub fn new() -> QueryDictTypeDetailResp {
        QueryDictTypeDetailResp {
            dict_id: 0,                  //字典主键
            dict_name: "".to_string(),   //字典名称
            dict_type: "".to_string(),   //字典类型
            status: 0,                   //状态（0：停用，1:正常）
            remark: "".to_string(),      //备注
            create_time: "".to_string(), //创建时间
            update_time: "".to_string(), //修改时间
        }
    }
}

/*
查询字典类型表列表请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryDictTypeListReq {
    pub page_no: u64,
    pub page_size: u64,
    pub dict_name: Option<String>, //字典名称
    pub dict_type: Option<String>, //字典类型
    pub status: Option<i8>,        //状态（0：停用，1:正常）
}

/*
查询字典类型表列表响应参数
*/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DictTypeListDataResp {
    pub dict_id: i64,        //字典主键
    pub dict_name: String,   //字典名称
    pub dict_type: String,   //字典类型
    pub status: i8,          //状态（0：停用，1:正常）
    pub remark: String,      //备注
    pub create_time: String, //创建时间
    pub update_time: String, //修改时间
}
impl DictTypeListDataResp {
    pub fn new() -> Vec<DictTypeListDataResp> {
        Vec::new()
    }
}
