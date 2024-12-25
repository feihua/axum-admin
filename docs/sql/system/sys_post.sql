create table sys_post
(
    id          bigint auto_increment comment '岗位id'
        primary key,
    post_code   varchar(64)                            not null comment '岗位编码',
    post_name   varchar(50)                            not null comment '岗位名称',
    sort        int          default 0                 not null comment '显示顺序',
    status      tinyint      default 0                 not null comment '部门状态（0：停用，1:正常）',
    remark      varchar(500) default ''                not null comment '备注',
    create_time datetime     default CURRENT_TIMESTAMP not null comment '创建时间',
    update_time datetime     default CURRENT_TIMESTAMP not null on update CURRENT_TIMESTAMP comment '更新时间'
) comment = '岗位信息表';