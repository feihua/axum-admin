create table sys_notice
(
    id             bigint auto_increment comment '公告ID'
        primary key,
    notice_title   varchar(50)                            not null comment '公告标题',
    notice_type    tinyint      default 1                 not null comment '公告类型（1:通知,2:公告）',
    notice_content varchar(255) default ''                not null comment '公告内容',
    status         tinyint      default 0                 not null comment '公告状态（0:关闭,1:正常 ）',
    remark         varchar(255) default ''                not null comment '备注',
    create_time    datetime     default CURRENT_TIMESTAMP not null comment '创建时间',
    update_time    datetime     default CURRENT_TIMESTAMP not null on update CURRENT_TIMESTAMP comment '修改时间'
) comment '通知公告表';




