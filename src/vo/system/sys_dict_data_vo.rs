// author：刘飞华
// createTime：2024/12/25 10:01:11

use serde::{Deserialize, Serialize};

/*
添加字典数据表请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddDictDataReq {
    pub dict_sort: i32,         //字典排序
    pub dict_label: String,     //字典标签
    pub dict_value: String,     //字典键值
    pub dict_type: String,      //字典类型
    pub css_class: String,      //样式属性（其他样式扩展）
    pub list_class: String,     //表格回显样式
    pub is_default: String,     //是否默认（Y是 N否）
    pub status: i8,             //状态（0：停用，1:正常）
    pub remark: Option<String>, //备注
}

/*
删除字典数据表请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteDictDataReq {
    pub ids: Vec<i64>,
}

/*
更新字典数据表请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateDictDataReq {
    pub dict_code: i64,         //字典编码
    pub dict_sort: i32,         //字典排序
    pub dict_label: String,     //字典标签
    pub dict_value: String,     //字典键值
    pub dict_type: String,      //字典类型
    pub css_class: String,      //样式属性（其他样式扩展）
    pub list_class: String,     //表格回显样式
    pub is_default: String,     //是否默认（Y是 N否）
    pub status: i8,             //状态（0：停用，1:正常）
    pub remark: Option<String>, //备注
}

/*
更新字典数据表状态请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDictDataStatusReq {
    pub ids: Vec<i64>,
    pub status: i8,
}

/*
查询字典数据表详情请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct QueryDictDataDetailReq {
    pub id: i64,
}

/*
查询字典数据表详情响应参数
*/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryDictDataDetailResp {
    pub dict_code: i64,      //字典编码
    pub dict_sort: i32,      //字典排序
    pub dict_label: String,  //字典标签
    pub dict_value: String,  //字典键值
    pub dict_type: String,   //字典类型
    pub css_class: String,   //样式属性（其他样式扩展）
    pub list_class: String,  //表格回显样式
    pub is_default: String,  //是否默认（Y是 N否）
    pub status: i8,          //状态（0：停用，1:正常）
    pub remark: String,      //备注
    pub create_time: String, //创建时间
    pub update_time: String, //修改时间
}

impl QueryDictDataDetailResp {
    pub fn new() -> QueryDictDataDetailResp {
        QueryDictDataDetailResp {
            dict_code: 0,                //字典编码
            dict_sort: 0,                //字典排序
            dict_label: "".to_string(),  //字典标签
            dict_value: "".to_string(),  //字典键值
            dict_type: "".to_string(),   //字典类型
            css_class: "".to_string(),   //样式属性（其他样式扩展）
            list_class: "".to_string(),  //表格回显样式
            is_default: "".to_string(),  //是否默认（Y是 N否）
            status: 0,                   //状态（0：停用，1:正常）
            remark: "".to_string(),      //备注
            create_time: "".to_string(), //创建时间
            update_time: "".to_string(), //修改时间
        }
    }
}

/*
查询字典数据表列表请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryDictDataListReq {
    pub page_no: u64,
    pub page_size: u64,
    pub dict_label: Option<String>, //字典标签
    pub dict_value: Option<String>, //字典键值
    pub dict_type: Option<String>,  //字典类型
    pub status: Option<i8>,         //状态（0：停用，1:正常）
}

/*
查询字典数据表列表响应参数
*/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DictDataListDataResp {
    pub dict_code: i64,      //字典编码
    pub dict_sort: i32,      //字典排序
    pub dict_label: String,  //字典标签
    pub dict_value: String,  //字典键值
    pub dict_type: String,   //字典类型
    pub css_class: String,   //样式属性（其他样式扩展）
    pub list_class: String,  //表格回显样式
    pub is_default: String,  //是否默认（Y是 N否）
    pub status: i8,          //状态（0：停用，1:正常）
    pub remark: String,      //备注
    pub create_time: String, //创建时间
    pub update_time: String, //修改时间
}
impl DictDataListDataResp {
    pub fn new() -> Vec<DictDataListDataResp> {
        Vec::new()
    }
}
