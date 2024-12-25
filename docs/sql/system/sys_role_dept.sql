create table sys_role_dept
(
    role_id bigint not null comment '角色id',
    dept_id bigint not null comment '部门id',
    primary key (role_id, dept_id)
) comment = '角色和部门关联表';
