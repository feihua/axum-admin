create table sys_user_role
(
    id          bigint auto_increment comment '主键'
        primary key,
    user_id     bigint   default 0                 not null comment '用户ID',
    role_id     bigint                             not null comment '角色ID'
)
    comment '角色用户关联表';
INSERT INTO sys_user_role (user_id, role_id) VALUES (1, 1)