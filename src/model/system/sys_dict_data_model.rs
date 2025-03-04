// Code generated by https://github.com/feihua/code_cli
// author：刘飞华
// createTime：2024/12/25 10:01:11

use crate::vo::system::sys_dict_data_vo::QueryDictDataListReq;
use rbatis::rbdc::datetime::DateTime;
use rbatis::RBatis;
use serde::{Deserialize, Serialize};
/*
 *字典数据表
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DictData {
    pub dict_code: Option<i64>,        //字典编码
    pub dict_sort: i32,                //字典排序
    pub dict_label: String,            //字典标签
    pub dict_value: String,            //字典键值
    pub dict_type: String,             //字典类型
    pub css_class: String,             //样式属性（其他样式扩展）
    pub list_class: String,            //表格回显样式
    pub is_default: String,            //是否默认（Y是 N否）
    pub status: i8,                    //状态（0：停用，1:正常）
    pub remark: String,                //备注
    pub create_time: Option<DateTime>, //创建时间
    pub update_time: Option<DateTime>, //修改时间
}

/*
 *字典数据表基本操作
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
rbatis::crud!(DictData {}, "sys_dict_data");

/*
 *根据id查询字典数据表
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
impl_select!(DictData{select_by_id(dict_code:&i64) -> Option => "`where dict_code = #{dict_code} limit 1`"}, "sys_dict_data");

/*
 *根据dict_type和dict_label查询字典数据表
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
impl_select!(DictData{select_by_dict_label(dict_type:&str, dict_label:&str) -> Option => "`where dict_type = #{dict_type} and dict_label = #{dict_label}`"}, "sys_dict_data");

/*
 *根据dict_type和dict_value查询字典数据表
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
impl_select!(DictData{select_by_dict_value(dict_type:&str, dict_value:&str) -> Option => "`where dict_type = #{dict_type} and dict_value = #{dict_value}`"}, "sys_dict_data");

/*
 *分页查询字典数据表
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
impl_select_page!(DictData{select_page() =>"
     if !sql.contains('count'):
       order by create_time desc"
},"sys_dict_data");

/*
 *根据条件分页查询字典数据表
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
impl_select_page!(DictData{select_dict_data_list(req:&QueryDictDataListReq) =>"
    where 1=1
     if req.dict_label != null && req.dict_label != '':
      ` and dict_label = #{req.dict_label} `
     if req.dict_type != null && req.dict_type != '':
      ` and dict_type = #{req.dict_type} `
     if req.status != 2:
      ` and status = #{req.status} `
     if !sql.contains('count'):
      ` order by create_time desc"
},"sys_dict_data");

/*
 *同步修改字典类型
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
#[sql("update sys_dict_data set dict_type = ? where dict_type = ?")]
pub async fn update_dict_data_type(
    rb: &RBatis,
    new_dict_type: &str,
    old_dict_type: &str,
) -> Option<i64> {
    impled!()
}

/*
 *查询字典数据
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
#[sql("select count(1) from sys_dict_data where dict_type= ?")]
pub async fn count_dict_data_by_type(rb: &RBatis, dict_type: &str) -> rbatis::Result<i64> {
    impled!()
}
