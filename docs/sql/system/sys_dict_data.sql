create table sys_dict_data
(
    dict_code   bigint                                 not null auto_increment    comment '字典编码',
    dict_sort   int          default 0                 not null comment '字典排序',
    dict_label  varchar(100) default ''                not null comment '字典标签',
    dict_value  varchar(100) default ''                not null comment '字典键值',
    dict_type   varchar(100) default ''                not null comment '字典类型',
    css_class   varchar(100) default ''                not null comment '样式属性（其他样式扩展）',
    list_class  varchar(100) default ''                not null comment '表格回显样式',
    is_default  char(1)      default 'N'               not null comment '是否默认（Y是 N否）',
    status      tinyint      default 0                 not null comment '状态（0：停用，1:正常）',
    remark      varchar(500) default ''                not null comment '备注',
    create_time datetime     default CURRENT_TIMESTAMP not null comment '创建时间',
    update_time datetime     default CURRENT_TIMESTAMP not null on update CURRENT_TIMESTAMP comment '修改时间',
    primary key (dict_code)
)comment = '字典数据表';


INSERT INTO sys_dict_data (dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, remark) VALUES (1, '男', '0', 'sys_user_sex', '1', '1', 'N', 1, '性别男');
INSERT INTO sys_dict_data (dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, remark) VALUES (2, '女', '1', 'sys_user_sex', '1', '1', 'N', 1, '性别女');
INSERT INTO sys_dict_data (dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, remark) VALUES (3, '未知', '2', 'sys_user_sex', '1', '1', 'N', 1, '性别未知');
INSERT INTO sys_dict_data (dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, remark) VALUES (1, '通知', '1', 'sys_notice_type', '1', '1', 'N', 1, '通知');
INSERT INTO sys_dict_data (dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, remark) VALUES (2, '公告', '2', 'sys_notice_type', '1', '1', 'N', 1, '公告');


