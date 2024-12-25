create table sys_dept
(
    id          bigint auto_increment comment '部门id'
        primary key,
    parent_id   bigint(20)  default 0                 not null comment '父部门id',
    ancestors   varchar(50) default ''                not null comment '祖级列表',
    dept_name   varchar(30) default ''                not null comment '部门名称',
    sort        int(4)      default 0                 not null comment '显示顺序',
    leader      varchar(20) default ''                not null comment '负责人',
    phone       varchar(11) default ''                not null comment '联系电话',
    email       varchar(50) default ''                not null comment '邮箱',
    status      tinyint     default 0                 not null comment '部门状态（0：停用，1:正常）',
    del_flag    tinyint     default 1                 not null comment '删除标志（0代表删除 1代表存在）',
    create_time datetime    default CURRENT_TIMESTAMP not null comment '创建时间',
    update_time datetime    default CURRENT_TIMESTAMP not null on update CURRENT_TIMESTAMP comment '修改时间'
) comment = '部门表';
