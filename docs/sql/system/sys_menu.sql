create table sys_menu
(
    id          bigint auto_increment comment '主键'
        primary key,
    menu_name   varchar(50)                            not null comment '菜单名称',
    menu_type   tinyint      default 1                 not null comment '菜单类型(1：目录   2：菜单   3：按钮)',
    `status`    tinyint      default 1                 not null comment '状态(1:正常，0:禁用)',
    sort        int          default 1                 not null comment '排序',
    parent_id   bigint       default 0                 not null comment '父ID',
    menu_url    varchar(255) default ''                not null comment '路由路径',
    api_url     varchar(255) default ''                not null comment '接口URL',
    menu_icon   varchar(255) default ''                not null comment '菜单图标',
    remark      varchar(255) default ''                not null comment '备注',
    create_time datetime     default CURRENT_TIMESTAMP not null comment '创建时间',
    update_time datetime     default CURRENT_TIMESTAMP not null on update CURRENT_TIMESTAMP comment '修改时间',
    constraint menu_name
        unique (menu_name)
)
    comment '菜单信息';


INSERT INTO sys_menu (id, menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES (1, '首页', 1, 1, 1, 0, '/home', '', 'SmileOutlined', '首页');
INSERT INTO sys_menu (id, menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES (2, '权限管理', 1, 1, 2, 0, '/system', '', 'SettingOutlined', '权限管理');

-- 配置用户信息权限
INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('用户信息管理', 2, 1, 1, 2, '/system/user', '', '', '用户信息管理');

select * from sys_menu where menu_name='用户信息管理';

INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('添加用户信息', 3, 1, 1, 3, '', '/api/system/user/addUser', '', '添加用户信息');
INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('删除用户信息', 3, 1, 2, 3, '', '/api/system/user/deleteUser', '', '删除用户信息');
INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('更新用户信息', 3, 1, 3, 3, '', '/api/system/user/updateUser', '', '更新用户信息');
INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('更新用户信息状态', 3, 1, 4, 3, '', '/api/system/user/updateUserStatus', '', '更新用户信息状态');
INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('更新用户密码', 3, 1, 4, 3, '', '/api/system/user/updateUserPassword', '', '更新用户密码');
INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('查询用户信息详情', 3, 1, 5, 3, '', '/api/system/user/queryUserDetail', '', '查询用户信息详情');
INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('查询用户信息列表', 3, 1, 6, 3, '', '/api/system/user/queryUserList', '', '查询用户信息列表');
INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('用户登录', 3, 1, 7, 3, '', '/api/system/user/login', '', '用户登录');
INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('查询用户菜单列表', 3, 1, 8, 3, '', '/api/system/user/queryUserMenu', '', '查询用户菜单列表');
INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('查询用户角色信息', 3, 1, 9, 3, '', '/api/system/user/queryUserRole', '', '查询用户角色信息');
INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('更新用户角色信息', 3, 1, 10, 3, '', '/api/system/user/updateUserRole', '', '更新用户角色信息');

-- 配置角色信息权限
INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('角色信息管理', 2, 1, 1, 2, '/system/role', '', '', '角色信息管理');

select * from sys_menu where menu_name='角色信息管理';

INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('添加角色信息', 3, 1, 1, 15, '', '/api/system/role/addRole', '', '添加角色信息');
INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('删除角色信息', 3, 1, 2, 15, '', '/api/system/role/deleteRole', '', '删除角色信息');
INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('更新角色信息', 3, 1, 3, 15, '', '/api/system/role/updateRole', '', '更新角色信息');
INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('更新角色信息状态', 3, 1, 4, 15, '', '/api/system/role/updateRoleStatus', '', '更新角色信息状态');
INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('查询角色信息详情', 3, 1, 5, 15, '', '/api/system/role/queryRoleDetail', '', '查询角色信息详情');
INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('查询角色信息列表', 3, 1, 6, 15, '', '/api/system/role/queryRoleList', '', '查询角色信息列表');
INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('查询角色菜单列表', 3, 1, 7, 15, '', '/api/system/role/queryRoleMenu', '', '查询角色菜单列表');
INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('更新角色菜单信息', 3, 1, 8, 15, '', '/api/system/role/updateRoleMenu', '', '更新角色菜单信息');

-- 配置菜单信息权限
INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('菜单信息管理', 2, 1, 1, 2, '/system/menu', '', '', '菜单信息管理');

select * from sys_menu where menu_name='菜单信息管理';

INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('添加菜单信息', 3, 1, 1, 24, '', '/api/system/menu/addMenu', '', '添加菜单信息');
INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('删除菜单信息', 3, 1, 2, 24, '', '/api/system/menu/deleteMenu', '', '删除菜单信息');
INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('更新菜单信息', 3, 1, 3, 24, '', '/api/system/menu/updateMenu', '', '更新菜单信息');
INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('更新菜单信息状态', 3, 1, 4, 24, '', '/api/system/menu/updateMenuStatus', '', '更新菜单信息状态');
INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('查询菜单信息详情', 3, 1, 5, 24, '', '/api/system/menu/queryMenuDetail', '', '查询菜单信息详情');
INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('查询菜单信息列表', 3, 1, 6, 24, '', '/api/system/menu/queryMenuList', '', '查询菜单信息列表');





