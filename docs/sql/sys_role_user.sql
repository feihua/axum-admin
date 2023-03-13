create table sys_role_user
(
    id           bigint unsigned auto_increment comment '主键'
        primary key,
    gmt_create   datetime         default CURRENT_TIMESTAMP not null comment '创建时间',
    gmt_modified datetime         default CURRENT_TIMESTAMP not null comment '修改时间',
    status_id    tinyint unsigned default 1                 not null comment '状态(1:正常，0:禁用)',
    sort         int unsigned     default 1                 not null comment '排序',
    role_id      bigint unsigned                            not null comment '角色ID',
    user_id      bigint unsigned  default 0                 not null comment '用户ID'
)
    comment '角色用户关联表' charset = utf8mb4;

INSERT INTO sys_role_user (id, gmt_create, gmt_modified, status_id, sort, role_id, user_id) VALUES (1145889062897147910, '2022-07-15 14:13:46', '2022-07-15 14:13:46', 1, 1, 3, 2);
INSERT INTO sys_role_user (id, gmt_create, gmt_modified, status_id, sort, role_id, user_id) VALUES (1145889062897147923, '2022-07-30 16:51:55', '2022-07-30 16:51:55', 1, 1, 3, 12);
INSERT INTO sys_role_user (id, gmt_create, gmt_modified, status_id, sort, role_id, user_id) VALUES (1145889062897147931, '2022-07-30 17:03:55', '2022-07-30 17:03:55', 1, 1, 4, 13);
INSERT INTO sys_role_user (id, gmt_create, gmt_modified, status_id, sort, role_id, user_id) VALUES (1145889062897147932, '2022-07-30 17:03:55', '2022-07-30 17:03:55', 1, 1, 3, 13);
INSERT INTO sys_role_user (id, gmt_create, gmt_modified, status_id, sort, role_id, user_id) VALUES (1145889062897147933, '2022-07-30 17:03:55', '2022-07-30 17:03:55', 1, 1, 1, 13);
