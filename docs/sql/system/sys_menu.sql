create table sys_menu
(
    id          bigint auto_increment comment '主键'
        primary key,
    menu_name   varchar(50)                            not null comment '菜单名称',
    menu_type   tinyint      default 1                 not null comment '菜单类型(1：目录   2：菜单   3：按钮)',
    visible     tinyint      default 1                 not null comment '显示状态（0:隐藏, 显示:1）',
    `status`    tinyint      default 1                 not null comment '菜单状态(1:正常，0:禁用)',
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
INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('用户管理', 2, 1, 1, 2, '/system/user', '', '', '用户信息管理');

select * from sys_menu where menu_name='用户管理';

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
INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('角色管理', 2, 1, 2, 2, '/system/role', '', '', '角色信息管理');

select * from sys_menu where menu_name='角色管理';

INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('添加角色信息', 3, 1, 1, 15, '', '/api/system/role/addRole', '', '添加角色信息');
INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('删除角色信息', 3, 1, 2, 15, '', '/api/system/role/deleteRole', '', '删除角色信息');
INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('更新角色信息', 3, 1, 3, 15, '', '/api/system/role/updateRole', '', '更新角色信息');
INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('更新角色信息状态', 3, 1, 4, 15, '', '/api/system/role/updateRoleStatus', '', '更新角色信息状态');
INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('查询角色信息详情', 3, 1, 5, 15, '', '/api/system/role/queryRoleDetail', '', '查询角色信息详情');
INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('查询角色信息列表', 3, 1, 6, 15, '', '/api/system/role/queryRoleList', '', '查询角色信息列表');
INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('查询角色菜单列表', 3, 1, 7, 15, '', '/api/system/role/queryRoleMenu', '', '查询角色菜单列表');
INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('更新角色菜单信息', 3, 1, 8, 15, '', '/api/system/role/updateRoleMenu', '', '更新角色菜单信息');

-- 配置菜单信息权限
INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('菜单管理', 2, 1, 3, 2, '/system/menu', '', '', '菜单信息管理');

select * from sys_menu where menu_name='菜单管理';

INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('添加菜单', 3, 1, 1, 24, '', '/api/system/menu/addMenu', '', '添加菜单');
INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('删除菜单', 3, 1, 2, 24, '', '/api/system/menu/deleteMenu', '', '删除菜单');
INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('更新菜单', 3, 1, 3, 24, '', '/api/system/menu/updateMenu', '', '更新菜单');
INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('更新菜单状态', 3, 1, 4, 24, '', '/api/system/menu/updateMenuStatus', '', '更新菜单状态');
INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('查询菜单详情', 3, 1, 5, 24, '', '/api/system/menu/queryMenuDetail', '', '查询菜单详情');
INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('查询菜单列表', 3, 1, 6, 24, '', '/api/system/menu/queryMenuList', '', '查询菜单列表');
INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('查询菜单树', 3, 1, 6, 24, '', '/api/system/menu/queryMenuListSimple', '', '查询菜单树');


-- 配置部门权限
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('部门管理', 2, 1, 4, 2, '/system/dept', '', '', '部门管理');

select * from sys_menu where menu_name='部门管理';

INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('添加部门', 3, 1, 1, 32, '', '/api/system/dept/addDept', '', '添加部门');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('删除部门', 3, 1, 2, 32, '', '/api/system/dept/deleteDept', '', '删除部门');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('更新部门', 3, 1, 3, 32, '', '/api/system/dept/updateDept', '', '更新部门');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('更新部门状态', 3, 1, 4, 32, '', '/api/system/dept/updateDeptStatus', '', '更新部门状态');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('查询部门详情', 3, 1, 5, 32, '', '/api/system/dept/queryDeptDetail', '', '查询部门详情');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('查询部门列', 3, 1, 6, 32, '', '/api/system/dept/queryDeptList', '', '查询部门列');

-- 配置岗位权限
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('岗位管理', 2, 1, 5, 2, '/system/post', '', '', '岗位管理');

select * from sys_menu where menu_name='岗位管理';

INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('添加岗位', 3, 1, 1, 39, '', '/api/system/post/addPost', '', '添加岗位');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('删除岗位', 3, 1, 2, 39, '', '/api/system/post/deletePost', '', '删除岗位');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('更新岗位', 3, 1, 3, 39, '', '/api/system/post/updatePost', '', '更新岗位');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('更新岗位状态', 3, 1, 4, 39, '', '/api/system/post/updatePostStatus', '', '更新岗位状态');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('查询岗位详情', 3, 1, 5, 39, '', '/api/system/post/queryPostDetail', '', '查询岗位详情');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('查询岗位列', 3, 1, 6, 39, '', '/api/system/post/queryPostList', '', '查询岗位列');

-- 配置字典类型权限
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('字典类型', 2, 1, 6, 2, '/system/dictType', '', '', '字典类型管理');

select * from sys_menu where menu_name='字典类型';

INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('添加字典类型', 3, 1, 1, 46, '', '/api/system/dictType/addDictType', '', '添加字典类型');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('删除字典类型', 3, 1, 2, 46, '', '/api/system/dictType/deleteDictType', '', '删除字典类型');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('更新字典类型', 3, 1, 3, 46, '', '/api/system/dictType/updateDictType', '', '更新字典类型');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('更新字典类型状态', 3, 1, 4, 46, '', '/api/system/dictType/updateDictTypeStatus', '', '更新字典类型状态');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('查询字典类型详情', 3, 1, 5, 46, '', '/api/system/dictType/queryDictTypeDetail', '', '查询字典类型详情');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('查询字典类型列', 3, 1, 6, 46, '', '/api/system/dictType/queryDictTypeList', '', '查询字典类型列');

-- 配置字典数据权限
INSERT INTO sys_menu (menu_name, menu_type, status, visible, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('字典数据', 2, 1, 0,7, 2, '/system/dictData', '', '', '字典数据管理');

select * from sys_menu where menu_name='字典数据';

INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('添加字典数据', 3, 1, 1, 53, '', '/api/system/dictData/addDictData', '', '添加字典数据');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('删除字典数据', 3, 1, 2, 53, '', '/api/system/dictData/deleteDictData', '', '删除字典数据');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('更新字典数据', 3, 1, 3, 53, '', '/api/system/dictData/updateDictData', '', '更新字典数据');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('更新字典数据状态', 3, 1, 4, 53, '', '/api/system/dictData/updateDictDataStatus', '', '更新字典数据状态');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('查询字典数据详情', 3, 1, 5, 53, '', '/api/system/dictData/queryDictDataDetail', '', '查询字典数据详情');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('查询字典数据列', 3, 1, 6, 53, '', '/api/system/dictData/queryDictDataList', '', '查询字典数据列');

-- 配置通知公告权限
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('通知公告', 2, 1, 8, 2, '/system/notice', '', '', '通知公告管理');

select * from sys_menu where menu_name='通知公告';

INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('添加通知公告', 3, 1, 1, 60, '', '/api/system/notice/addNotice', '', '添加通知公告');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('删除通知公告', 3, 1, 2, 60, '', '/api/system/notice/deleteNotice', '', '删除通知公告');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('更新通知公告', 3, 1, 3, 60, '', '/api/system/notice/updateNotice', '', '更新通知公告');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('更新通知公告状态', 3, 1, 4, 60, '', '/api/system/notice/updateNoticeStatus', '', '更新通知公告状态');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('查询通知公告详情', 3, 1, 5, 60, '', '/api/system/notice/queryNoticeDetail', '', '查询通知公告详情');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('查询通知公告列', 3, 1, 6, 60, '', '/api/system/notice/queryNoticeList', '', '查询通知公告列');

INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('日志管理', 1, 1, 2, 0, '/log', '', 'SettingOutlined', '日志管理');
select * from sys_menu where menu_name='日志管理';

-- 配置系统访问记录权限
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('登录日志', 2, 1, 9, 66, '/system/loginLog', '', '', '系统访问记录管理');

select * from sys_menu where menu_name='登录日志';

INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('添加系统访问记录', 3, 1, 1, 68, '', '/api/system/loginLog/addLoginLog', '', '添加系统访问记录');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('删除系统访问记录', 3, 1, 2, 68, '', '/api/system/loginLog/deleteLoginLog', '', '删除系统访问记录');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('清空系统登录日志', 3, 1, 3, 68, '', '/api/system/loginLog/cleanLoginLog', '', '清空系统登录日志');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('更新系统访问记录状态', 3, 1, 4, 68, '', '/api/system/loginLog/updateLoginLogStatus', '', '更新系统访问记录状态');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('查询系统访问记录详情', 3, 1, 5, 68, '', '/api/system/loginLog/queryLoginLogDetail', '', '查询系统访问记录详情');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('查询系统访问记录列', 3, 1, 6, 68, '', '/api/system/loginLog/queryLoginLogList', '', '查询系统访问记录列');


-- 配置操作日志记录权限
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('操作日志', 2, 1, 10, 66, '/system/operateLog', '', '', '操作日志记录管理');

select * from sys_menu where menu_name='操作日志';

INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('添加操作日志记录', 3, 1, 1, 75, '', '/api/system/operateLog/addOperateLog', '', '添加操作日志记录');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('删除操作日志记录', 3, 1, 2, 75, '', '/api/system/operateLog/deleteOperateLog', '', '删除操作日志记录');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('清空操作日志记录', 3, 1, 3, 75, '', '/api/system/operateLog/cleanOperateLog', '', '清空操作日志记录');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('更新操作日志记录状态', 3, 1, 4, 75, '', '/api/system/operateLog/updateOperateLogStatus', '', '更新操作日志记录状态');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('查询操作日志记录详情', 3, 1, 5, 75, '', '/api/system/operateLog/queryOperateLogDetail', '', '查询操作日志记录详情');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('查询操作日志记录列', 3, 1, 6, 75, '', '/api/system/operateLog/queryOperateLogList', '', '查询操作日志记录列');


INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('其他', 1, 1, 3, 0, '/other', '', 'SettingOutlined', '其他');



