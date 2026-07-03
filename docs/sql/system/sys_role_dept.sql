create table sys_role_dept
(
    id      bigint auto_increment comment '主键'
        primary key,
    role_id bigint not null comment '角色id',
    dept_id bigint not null comment '部门id'
) comment = '角色和部门关联表';
