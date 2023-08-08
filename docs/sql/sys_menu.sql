create table sys_menu
(
    id          bigint auto_increment comment '主键'
        primary key,
    menu_name   varchar(50)                            not null comment '菜单名称',
    menu_type   tinyint      default 1                 not null comment '菜单类型(1：目录   2：菜单   3：按钮)',
    status_id   tinyint      default 1                 not null comment '状态(1:正常，0:禁用)',
    sort        int          default 1                 not null comment '排序',
    parent_id   bigint                                 not null comment '父ID',
    menu_url    varchar(255) default ''                null comment '路由路径',
    api_url     varchar(255) default ''                null comment '接口URL',
    menu_icon   varchar(255)                           null comment '菜单图标',
    remark      varchar(255)                           null comment '备注',
    create_time datetime     default CURRENT_TIMESTAMP not null comment '创建时间',
    update_time datetime     default CURRENT_TIMESTAMP not null on update CURRENT_TIMESTAMP comment '修改时间',
    constraint menu_name
        unique (menu_name)
)
    comment '菜单信息';

INSERT INTO sys_menu (id, menu_name, menu_type, status_id, sort, parent_id, menu_url, api_url, menu_icon, remark, create_time, update_time) VALUES (1, '首页', 1, 1, 0, 0, '/home', '', 'SmileOutlined', '首页', '2022-07-14 15:40:10', '2022-07-14 17:40:10');
INSERT INTO sys_menu (id, menu_name, menu_type, status_id, sort, parent_id, menu_url, api_url, menu_icon, remark, create_time, update_time) VALUES (2, '权限管理', 1, 1, 1, 0, '', '', 'SettingOutlined', '权限管理', '2022-07-14 17:40:10', '2022-07-14 17:40:10');
INSERT INTO sys_menu (id, menu_name, menu_type, status_id, sort, parent_id, menu_url, api_url, menu_icon, remark, create_time, update_time) VALUES (3, '用户管理', 2, 0, 3, 2, '/user', '/api/user_list', '', '用户管理', '2022-07-14 17:40:10', '2022-07-14 17:40:10');
INSERT INTO sys_menu (id, menu_name, menu_type, status_id, sort, parent_id, menu_url, api_url, menu_icon, remark, create_time, update_time) VALUES (4, '角色管理', 2, 1, 2, 2, '/role', '/api/role_list', '', '角色管理', '2022-07-14 17:40:10', '2022-07-14 17:40:10');
INSERT INTO sys_menu (id, menu_name, menu_type, status_id, sort, parent_id, menu_url, api_url, menu_icon, remark, create_time, update_time) VALUES (5, '菜单管理', 2, 1, 1, 2, '/menu', '/api/menu_list', '', '菜单管理', '2022-07-14 17:40:10', '2022-07-14 17:40:10');
INSERT INTO sys_menu (id, menu_name, menu_type, status_id, sort, parent_id, menu_url, api_url, menu_icon, remark, create_time, update_time) VALUES (6, '更新用户状态接口', 3, 1, 1, 3, '', '/api/update_user_status', '', '更新用户状态接口', '2022-07-14 17:40:10', '2022-07-14 17:40:10');
INSERT INTO sys_menu (id, menu_name, menu_type, status_id, sort, parent_id, menu_url, api_url, menu_icon, remark, create_time, update_time) VALUES (7, '保存用户弹窗', 3, 1, 1, 3, '', '/api/user_save_view', '', '保存用户弹窗', '2022-07-14 17:40:10', '2022-07-14 17:40:10');
INSERT INTO sys_menu (id, menu_name, menu_type, status_id, sort, parent_id, menu_url, api_url, menu_icon, remark, create_time, update_time) VALUES (8, '保存用户接口', 3, 1, 1, 3, '', '/api/user_save', '', '保存用户接口', '2022-07-14 17:40:10', '2022-07-14 17:40:10');
INSERT INTO sys_menu (id, menu_name, menu_type, status_id, sort, parent_id, menu_url, api_url, menu_icon, remark, create_time, update_time) VALUES (9, '删除用户接口', 3, 1, 1, 3, '', '/api/user_delete', '', '删除用户接口', '2022-07-14 17:40:11', '2022-07-14 17:40:11');
INSERT INTO sys_menu (id, menu_name, menu_type, status_id, sort, parent_id, menu_url, api_url, menu_icon, remark, create_time, update_time) VALUES (10, '更新用户弹窗', 3, 1, 1, 3, '', '/api/user_update_view', '', '更新用户弹窗', '2022-07-14 17:40:11', '2022-07-14 17:40:11');
INSERT INTO sys_menu (id, menu_name, menu_type, status_id, sort, parent_id, menu_url, api_url, menu_icon, remark, create_time, update_time) VALUES (11, '更新用户接口', 3, 1, 1, 3, '', '/api/user_update', '', '更新用户接口', '2022-07-14 17:40:11', '2022-07-14 17:40:11');
INSERT INTO sys_menu (id, menu_name, menu_type, status_id, sort, parent_id, menu_url, api_url, menu_icon, remark, create_time, update_time) VALUES (12, '更新用户密码弹窗', 3, 1, 1, 3, '', '/api/update_user_password_view', '', '更新用户密码弹窗', '2022-07-14 17:40:11', '2022-07-14 17:40:11');
INSERT INTO sys_menu (id, menu_name, menu_type, status_id, sort, parent_id, menu_url, api_url, menu_icon, remark, create_time, update_time) VALUES (13, '更新用户密码', 3, 1, 1, 3, '', '/api/update_user_password', '', '更新用户密码接口', '2022-07-14 17:40:11', '2022-07-26 21:58:51');
INSERT INTO sys_menu (id, menu_name, menu_type, status_id, sort, parent_id, menu_url, api_url, menu_icon, remark, create_time, update_time) VALUES (14, '设置角色弹窗', 3, 1, 1, 3, '', '/api/update_user_role_view', '', '设置角色弹窗', '2022-07-14 17:40:11', '2022-07-14 17:40:11');
INSERT INTO sys_menu (id, menu_name, menu_type, status_id, sort, parent_id, menu_url, api_url, menu_icon, remark, create_time, update_time) VALUES (15, '保存用户角色', 3, 1, 1, 3, '', '/api/update_user_role', '', '保存用户角色接口', '2022-07-14 17:40:11', '2022-07-14 17:40:11');
INSERT INTO sys_menu (id, menu_name, menu_type, status_id, sort, parent_id, menu_url, api_url, menu_icon, remark, create_time, update_time) VALUES (16, '用户关联的角色', 3, 1, 1, 3, '', '/api/query_user_role', '', '获取用户关联的角色', '2022-07-14 17:40:11', '2022-07-14 17:40:11');
INSERT INTO sys_menu (id, menu_name, menu_type, status_id, sort, parent_id, menu_url, api_url, menu_icon, remark, create_time, update_time) VALUES (17, '查询用户菜单接口', 3, 1, 1, 3, '', '/api/query_user_menu', '', '查询用户菜单接口', '2022-07-14 17:40:11', '2022-07-14 17:40:11');
INSERT INTO sys_menu (id, menu_name, menu_type, status_id, sort, parent_id, menu_url, api_url, menu_icon, remark, create_time, update_time) VALUES (18, '更新角色状态接口', 3, 1, 1, 4, '', '/api/update_role_status', '', '更新角色状态接口', '2022-07-14 17:40:11', '2022-07-14 17:40:11');
INSERT INTO sys_menu (id, menu_name, menu_type, status_id, sort, parent_id, menu_url, api_url, menu_icon, remark, create_time, update_time) VALUES (19, '保存角色弹窗', 3, 1, 1, 4, '', '/api/role_save_view', '', '保存角色弹窗', '2022-07-14 17:40:12', '2022-07-14 17:40:12');
INSERT INTO sys_menu (id, menu_name, menu_type, status_id, sort, parent_id, menu_url, api_url, menu_icon, remark, create_time, update_time) VALUES (20, '保存角色接口', 3, 1, 1, 4, '', '/api/role_save', '', '保存角色接口', '2022-07-14 17:40:12', '2022-07-14 17:40:12');
INSERT INTO sys_menu (id, menu_name, menu_type, status_id, sort, parent_id, menu_url, api_url, menu_icon, remark, create_time, update_time) VALUES (21, '删除角色接口', 3, 1, 1, 4, '', '/api/role_delete', '', '删除角色接口', '2022-07-14 17:40:12', '2022-07-14 17:40:12');
INSERT INTO sys_menu (id, menu_name, menu_type, status_id, sort, parent_id, menu_url, api_url, menu_icon, remark, create_time, update_time) VALUES (22, '修改角色弹窗', 3, 1, 1, 4, '', '/api/role_update_view', '', '修改角色弹窗', '2022-07-14 17:40:12', '2022-07-14 17:40:12');
INSERT INTO sys_menu (id, menu_name, menu_type, status_id, sort, parent_id, menu_url, api_url, menu_icon, remark, create_time, update_time) VALUES (23, '更新角色接口', 3, 1, 1, 4, '', '/api/role_update', '', '更新角色接口', '2022-07-14 17:40:12', '2022-07-14 17:40:12');
INSERT INTO sys_menu (id, menu_name, menu_type, status_id, sort, parent_id, menu_url, api_url, menu_icon, remark, create_time, update_time) VALUES (24, '设置权限弹窗', 3, 0, 1, 4, '', '/api/query_role_menu_view', '', '设置权限弹窗', '2022-07-14 17:40:12', '2022-07-14 17:40:12');
INSERT INTO sys_menu (id, menu_name, menu_type, status_id, sort, parent_id, menu_url, api_url, menu_icon, remark, create_time, update_time) VALUES (25, '菜单角色关联', 3, 1, 1, 4, '', '/api/query_role_menu', '', '菜单角色关联', '2022-07-14 17:40:12', '2022-07-14 17:40:12');
INSERT INTO sys_menu (id, menu_name, menu_type, status_id, sort, parent_id, menu_url, api_url, menu_icon, remark, create_time, update_time) VALUES (26, '保存角色菜单关联', 3, 1, 1, 4, '', '/api/update_role_menu', '', '角色菜单关联接口', '2022-07-14 17:40:12', '2022-07-14 17:40:12');
INSERT INTO sys_menu (id, menu_name, menu_type, status_id, sort, parent_id, menu_url, api_url, menu_icon, remark, create_time, update_time) VALUES (27, '更新菜单状态接口', 3, 1, 1, 5, '', '/api/update_menu_status', '', '更新菜单状态接口', '2022-07-14 17:40:12', '2022-07-14 17:40:12');
INSERT INTO sys_menu (id, menu_name, menu_type, status_id, sort, parent_id, menu_url, api_url, menu_icon, remark, create_time, update_time) VALUES (28, '保存菜单弹窗', 3, 1, 1, 5, '', '/api/menu_save_view', '', '保存菜单弹窗', '2022-07-14 17:40:12', '2022-07-14 17:40:12');
INSERT INTO sys_menu (id, menu_name, menu_type, status_id, sort, parent_id, menu_url, api_url, menu_icon, remark, create_time, update_time) VALUES (29, '保存菜单接口', 3, 1, 1, 5, '', '/api/menu_save', '', '保存菜单接口', '2022-07-14 17:40:12', '2022-07-14 17:40:12');
INSERT INTO sys_menu (id, menu_name, menu_type, status_id, sort, parent_id, menu_url, api_url, menu_icon, remark, create_time, update_time) VALUES (30, '删除菜单接口', 3, 1, 1, 5, '', '/api/menu_delete', '', '删除菜单接口', '2022-07-14 17:40:12', '2022-07-14 17:40:12');
INSERT INTO sys_menu (id, menu_name, menu_type, status_id, sort, parent_id, menu_url, api_url, menu_icon, remark, create_time, update_time) VALUES (31, '修改菜单弹窗', 3, 1, 1, 5, '', '/api/menu_update_view', '', '修改菜单弹窗', '2022-07-14 17:40:13', '2022-07-14 17:40:13');
INSERT INTO sys_menu (id, menu_name, menu_type, status_id, sort, parent_id, menu_url, api_url, menu_icon, remark, create_time, update_time) VALUES (32, '更新菜单接口', 3, 1, 1, 5, '', '/api/menu_update', '', '更新菜单接口', '2022-07-14 17:40:13', '2022-07-14 17:40:13');
INSERT INTO sys_menu (id, menu_name, menu_type, status_id, sort, parent_id, menu_url, api_url, menu_icon, remark, create_time, update_time) VALUES (63, '日志管理', 1, 1, 1, 0, '/log1', '', 'Setting', '', '2023-01-14 15:23:42', '2023-01-14 15:23:42');
INSERT INTO sys_menu (id, menu_name, menu_type, status_id, sort, parent_id, menu_url, api_url, menu_icon, remark, create_time, update_time) VALUES (64, '登录日志', 2, 1, 1, 63, '/log', '', 'Setting', '', '2023-01-14 15:24:07', '2023-01-14 15:24:07');
INSERT INTO sys_menu (id, menu_name, menu_type, status_id, sort, parent_id, menu_url, api_url, menu_icon, remark, create_time, update_time) VALUES (65, '常用图表', 1, 1, 1, 0, '/line1', '', 'Setting', '', '2023-01-14 15:24:51', '2023-01-14 15:24:51');
INSERT INTO sys_menu (id, menu_name, menu_type, status_id, sort, parent_id, menu_url, api_url, menu_icon, remark, create_time, update_time) VALUES (66, '饼图', 2, 1, 1, 65, '/bar', '', 'Setting', '', '2023-01-14 15:25:15', '2023-01-14 15:25:15');
INSERT INTO sys_menu (id, menu_name, menu_type, status_id, sort, parent_id, menu_url, api_url, menu_icon, remark, create_time, update_time) VALUES (67, '线图', 2, 1, 1, 65, '/line', '', 'Setting', '', '2023-01-14 15:25:38', '2023-01-14 15:25:38');
INSERT INTO sys_menu (id, menu_name, menu_type, status_id, sort, parent_id, menu_url, api_url, menu_icon, remark, create_time, update_time) VALUES (68, '柱状图', 2, 1, 1, 65, '/pie', '', 'Setting', '', '2023-01-14 15:25:52', '2023-01-14 15:25:52');
INSERT INTO sys_menu (id, menu_name, menu_type, status_id, sort, parent_id, menu_url, api_url, menu_icon, remark, create_time, update_time) VALUES (69, '个人中心', 1, 1, 1, 0, '/center1', '', 'Setting', '', '2023-01-14 15:26:47', '2023-01-14 15:26:47');
INSERT INTO sys_menu (id, menu_name, menu_type, status_id, sort, parent_id, menu_url, api_url, menu_icon, remark, create_time, update_time) VALUES (72, '个人信息', 2, 1, 1, 69, '/center', '', 'Setting', '', '2023-01-14 15:27:29', '2023-01-15 18:35:48');
INSERT INTO sys_menu (id, menu_name, menu_type, status_id, sort, parent_id, menu_url, api_url, menu_icon, remark, create_time, update_time) VALUES (73, '个人设置', 2, 1, 1, 69, '/setting', '', 'Setting', '', '2023-01-14 15:27:47', '2023-01-14 15:27:47');
