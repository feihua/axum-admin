create table sys_role
(
    id           bigint unsigned auto_increment comment '主键'
        primary key,
    gmt_create   datetime         default CURRENT_TIMESTAMP not null comment '创建时间',
    gmt_modified datetime         default CURRENT_TIMESTAMP not null comment '修改时间',
    status_id    tinyint unsigned default 1                 not null comment '状态(1:正常，0:禁用)',
    sort         int unsigned     default 1                 not null comment '排序',
    role_name    varchar(50)                                not null comment '名称',
    remark       varchar(255)                               not null comment '备注',
    constraint role_name
        unique (role_name)
)
    comment '角色信息' charset = utf8mb4;

create index name_status_index
    on sys_role (role_name, status_id);

INSERT INTO sys_role (id, gmt_create, gmt_modified, status_id, sort, role_name, remark) VALUES (1, '2018-02-06 15:47:52', '2018-03-30 15:12:07', 1, 1, '超级管理员', '全部权限');
INSERT INTO sys_role (id, gmt_create, gmt_modified, status_id, sort, role_name, remark) VALUES (3, '2018-12-28 18:23:38', '2018-12-28 18:23:38', 1, 1, '演示角色', '仅有查看功能');
INSERT INTO sys_role (id, gmt_create, gmt_modified, status_id, sort, role_name, remark) VALUES (4, '2022-07-14 16:05:07', '2022-07-15 16:16:17', 0, 1, '121', '121211');
