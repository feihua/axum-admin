create table sys_dict_type
(
    dict_id     bigint                                 not null auto_increment comment '字典主键',
    dict_name   varchar(100) default ''                not null comment '字典名称',
    dict_type   varchar(100) default ''                not null comment '字典类型',
    status      tinyint      default 0                 not null comment '状态（0：停用，1:正常）',
    remark      varchar(500) default ''                not null comment '备注',
    create_time datetime     default CURRENT_TIMESTAMP not null comment '创建时间',
    update_time datetime     default CURRENT_TIMESTAMP not null on update CURRENT_TIMESTAMP comment '修改时间',
    primary key (dict_id),
    unique (dict_type)
) comment = '字典类型表';

INSERT INTO sys_dict_type (dict_name, dict_type, status, remark) VALUES ('用户性别', 'sys_user_sex', 1, '用户性别列表');
INSERT INTO sys_dict_type (dict_name, dict_type, status, remark) VALUES ('通知类型', 'sys_notice_type', 1, '通知类型列表');
