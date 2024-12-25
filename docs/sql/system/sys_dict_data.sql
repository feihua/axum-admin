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
    status      tinyint      default 0                 not null comment '门状态（0：停用，1:正常）',
    remark      varchar(500) default ''                not null comment '备注',
    create_time datetime     default CURRENT_TIMESTAMP not null comment '创建时间',
    update_time datetime     default CURRENT_TIMESTAMP not null on update CURRENT_TIMESTAMP comment '修改时间',
    primary key (dict_code)
)comment = '字典数据表';




