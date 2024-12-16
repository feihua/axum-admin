create table sys_role
(
    id          bigint auto_increment comment '主键'
        primary key,
    role_name   varchar(50)                        not null comment '名称',
    status_id   tinyint  default 1                 not null comment '状态(1:正常，0:禁用)',
    sort        int      default 1                 not null comment '排序',
    remark      varchar(255)                       not null comment '备注',
    create_time datetime default CURRENT_TIMESTAMP not null comment '创建时间',
    update_time datetime default CURRENT_TIMESTAMP not null on update CURRENT_TIMESTAMP comment '修改时间',
    constraint role_name
        unique (role_name)
)
    comment '角色信息';

create index name_status_index
    on sys_role (role_name, status_id);

INSERT INTO sys_role (id, role_name, status_id, sort, remark, create_time, update_time) VALUES (1, '超级管理员', 1, 1, '全部权限', '2018-02-06 15:47:52', '2018-03-30 15:12:07');
INSERT INTO sys_role (id, role_name, status_id, sort, remark, create_time, update_time) VALUES (3, '演示角色', 1, 1, '仅有查看功能', '2018-12-28 18:23:38', '2018-12-28 18:23:38');
INSERT INTO sys_role (id, role_name, status_id, sort, remark, create_time, update_time) VALUES (4, '121', 0, 1, '121211', '2022-07-14 16:05:07', '2022-07-15 16:16:17');
