use crate::model::system::sys_user_model::User;
use rbatis::RBatis;
use rbs::value;
use tracing::{info, warn};

pub async fn sync_tables(rb: &RBatis) {
    // 执行 DDL
    rb.exec(
        "
create table if not exists sys_dept
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
    update_time datetime     null on update CURRENT_TIMESTAMP comment '修改时间'
) comment = '部门表';

create table if not exists sys_dict_data
(
    id   bigint                                 not null auto_increment    comment '字典编码',
    dict_sort   int          default 0                 not null comment '字典排序',
    dict_label  varchar(100) default ''                not null comment '字典标签',
    dict_value  varchar(100) default ''                not null comment '字典键值',
    dict_type   varchar(100) default ''                not null comment '字典类型',
    css_class   varchar(100) default ''                not null comment '样式属性（其他样式扩展）',
    list_class  varchar(100) default ''                not null comment '表格回显样式',
    is_default  char(1)      default 'N'               not null comment '是否默认（Y是 N否）',
    status      tinyint      default 0                 not null comment '状态（0：停用，1:正常）',
    remark      varchar(500) default ''                not null comment '备注',
    create_time datetime     default CURRENT_TIMESTAMP not null comment '创建时间',
    update_time datetime      null on update CURRENT_TIMESTAMP comment '修改时间',
    primary key (id)
)comment = '字典数据表';

create table if not exists sys_dict_type
(
    id     bigint                                 not null auto_increment comment '字典主键',
    dict_name   varchar(100) default ''                not null comment '字典名称',
    dict_type   varchar(100) default ''                not null comment '字典类型',
    status      tinyint      default 0                 not null comment '状态（0：停用，1:正常）',
    remark      varchar(500) default ''                not null comment '备注',
    create_time datetime     default CURRENT_TIMESTAMP not null comment '创建时间',
    update_time datetime      null on update CURRENT_TIMESTAMP comment '修改时间',
    primary key (id),
    unique (dict_type)
) comment = '字典类型表';

create table if not exists sys_login_log
(
    id             bigint auto_increment comment '访问ID'
        primary key,
    login_name     varchar(50)  default ''                not null comment '登录账号',
    ipaddr         varchar(128) default ''                not null comment '登录IP地址',
    login_location varchar(255) default ''                not null comment '登录地点',
    platform       varchar(50)  default ''                not null comment '平台信息',
    browser        varchar(50)  default ''                not null comment '浏览器类型',
    version        varchar(50)  default ''                not null comment '浏览器版本',
    os             varchar(50)  default ''                not null comment '操作系统',
    arch           varchar(50)  default ''                not null comment '体系结构信息',
    engine         varchar(50)  default ''                not null comment '渲染引擎信息',
    engine_details varchar(50)  default ''                not null comment '渲染引擎详细信息',
    extra          varchar(50)  default ''                not null comment '其他信息（可选）',
    status         tinyint      default 0                 not null comment '登录状态(0:失败,1:成功)',
    msg            varchar(255) default ''                not null comment '提示消息',
    login_time     datetime     default CURRENT_TIMESTAMP not null comment '访问时间'
) comment = '系统访问记录';

create table if not exists sys_menu
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
    update_time datetime      null on update CURRENT_TIMESTAMP comment '修改时间',
    constraint menu_name
        unique (menu_name)
)
    comment '菜单信息';

create table if not exists sys_notice
(
    id             bigint auto_increment comment '公告ID'
        primary key,
    notice_title   varchar(50)                            not null comment '公告标题',
    notice_type    tinyint      default 1                 not null comment '公告类型（1:通知,2:公告）',
    notice_content varchar(255) default ''                not null comment '公告内容',
    status         tinyint      default 0                 not null comment '公告状态（0:关闭,1:正常 ）',
    remark         varchar(255) default ''                not null comment '备注',
    create_time    datetime     default CURRENT_TIMESTAMP not null comment '创建时间',
    update_time    datetime      null on update CURRENT_TIMESTAMP comment '修改时间'
) comment '通知公告表';

create table if not exists sys_operate_log
(
    id               bigint auto_increment comment '日志主键'
        primary key,
    title            varchar(50)   default '' comment '模块标题',
    business_type    tinyint       default 0 comment '业务类型（0其它 1新增 2修改 3删除）',
    method           varchar(200)  default '' comment '方法名称',
    request_method   varchar(10)   default '' comment '请求方式',
    operator_type    tinyint       default 0 comment '操作类别（0其它 1后台用户 2手机端用户）',
    operate_name     varchar(50)   default '' comment '操作人员',
    dept_name        varchar(50)   default '' comment '部门名称',
    operate_url      varchar(255)  default '' comment '请求URL',
    operate_ip       varchar(128)  default '' comment '主机地址',
    operate_location varchar(255)  default '' comment '操作地点',
    operate_param    varchar(2000) default '' comment '请求参数',
    json_result      varchar(2000) default '' comment '返回参数',
    status           tinyint       default 0 comment '操作状态(0:异常,正常)',
    error_msg        varchar(2000) default '' comment '错误消息',
    operate_time     datetime      default CURRENT_TIMESTAMP not null comment '操作时间',
    cost_time        bigint(20)    default 0 comment '消耗时间'

) comment = '操作日志记录';

create table if not exists sys_post
(
    id          bigint auto_increment comment '岗位id'
        primary key,
    post_code   varchar(64)                            not null comment '岗位编码',
    post_name   varchar(50)                            not null comment '岗位名称',
    sort        int          default 0                 not null comment '显示顺序',
    status      tinyint      default 0                 not null comment '岗位状态（0：停用，1:正常）',
    remark      varchar(500) default ''                not null comment '备注',
    create_time datetime     default CURRENT_TIMESTAMP not null comment '创建时间',
    update_time datetime null on update CURRENT_TIMESTAMP comment '更新时间'
) comment = '岗位信息表';

create table if not exists sys_role
(
    id          bigint auto_increment comment '主键'
        primary key,
    role_name   varchar(50)                            not null comment '名称',
    role_key    varchar(100) default ''                not null comment '角色权限字符串',
    data_scope  tinyint      default 1                 not null comment '数据范围（1：全部数据权限 2：自定数据权限 3：本部门数据权限 4：本部门及以下数据权限）',
    status      tinyint      default 1                 not null comment '状态(1:正常，0:禁用)',
    remark      varchar(255)                           not null comment '备注',
    del_flag    tinyint      default 1                 not null comment '删除标志（0代表删除 1代表存在）',
    create_time datetime     default CURRENT_TIMESTAMP not null comment '创建时间',
    update_time datetime      null on update CURRENT_TIMESTAMP comment '修改时间',
    constraint role_name
        unique (role_name)
) comment '角色信息';

create table if not exists sys_role_dept
(
    id      bigint auto_increment comment '主键'
        primary key,
    role_id bigint not null comment '角色id',
    dept_id bigint not null comment '部门id'
) comment = '角色和部门关联表';

create table if not exists sys_role_menu
(
    id          bigint auto_increment comment '主键'
        primary key,
    role_id     bigint                             not null comment '角色ID',
    menu_id     bigint                             not null comment '菜单ID'
)
    comment '菜单角色关联表';

create table if not exists sys_user
(
    id              bigint auto_increment comment '主键'
        primary key,
    mobile          char(11)     default ''                not null comment '手机号码',
    user_name       varchar(50)                            not null comment '用户账号',
    nick_name       varchar(30)                            not null comment '用户昵称',
    user_type       varchar(2)   default '00'              not null comment '用户类型（00系统用户）',
    avatar          varchar(100) default ''                not null comment '头像路径',
    email           varchar(50)  default ''                not null comment '用户邮箱',
    password        varchar(64)                            not null comment '密码',
    status          tinyint      default 1                 not null comment '状态(1:正常，0:禁用)',
    dept_id         bigint       default 1                 not null comment '部门ID',
    login_ip        varchar(128) default ''                not null comment '最后登录IP',
    login_date      datetime comment '最后登录时间',
    login_browser   varchar(50)  default ''                not null comment '浏览器类型',
    login_os        varchar(50)  default ''                not null comment '操作系统',
    pwd_update_date datetime comment '密码最后更新时间',
    remark          varchar(255) null comment '备注',
    del_flag        tinyint      default 1                 not null comment '删除标志（0代表删除 1代表存在）',
    create_time     datetime     default CURRENT_TIMESTAMP not null comment '创建时间',
    update_time     datetime      null on update CURRENT_TIMESTAMP comment '修改时间',
    constraint AK_phone
        unique (mobile)
) comment '用户信息';

create table if not exists sys_user_post
(
    id          bigint auto_increment comment '主键'
        primary key,
    user_id bigint not null comment '用户id',
    post_id bigint not null comment '岗位id'
) comment = '用户与岗位关联表';

create table if not exists sys_user_role
(
    id          bigint auto_increment comment '主键'
        primary key,
    user_id     bigint   default 0                 not null comment '用户ID',
    role_id     bigint                             not null comment '角色ID'
)
    comment '角色用户关联表';

",
        vec![],
    )
    .await
    .unwrap();
}

pub async fn sync_tables_data(rb: &RBatis) {
    let vec1 = User::select_by_map(rb, value! {}).await.unwrap_or_default();
    if !vec1.is_empty() {
        warn!("数据已存在，不再生成数据");
        return;
    }

    rb.exec(
        "
INSERT INTO sys_user (id, mobile, user_name, nick_name, avatar, email, password, status, remark) VALUES (1, '18613030111', 'admin','admin', 'https://gw.alipayobjects.com/zos/antfincdn/XAosXuNZyF/BiazfanxmamNRoxxVxka.png','xx@qq.com','123456', 1,  '超级管理员');
INSERT INTO sys_user (id, mobile, user_name, nick_name, avatar, email, password, status, remark) VALUES (2, '18613030222', 'test', 'test', 'https://gw.alipayobjects.com/zos/antfincdn/XAosXuNZyF/BiazfanxmamNRoxxVxka.png','123@qq.com','123456', 1, '演示权限');
insert into sys_user_post(user_id, post_id) values ('1', '1');
insert into sys_user_post(user_id, post_id) values ('2', '2');
INSERT INTO sys_user_role (user_id, role_id) VALUES (1, 1);
INSERT INTO sys_role (id, role_name, role_key, status, remark) VALUES (1, '超级管理员', 'admin',1, '全部权限');
INSERT INTO sys_role (id, role_name, role_key, status, remark) VALUES (2, '演示角色', 'query',1,  '仅有查看功能');
INSERT INTO sys_role (id, role_name, role_key, status, remark) VALUES (3, '121', 'dev',0, '121211');
INSERT INTO sys_post (post_code, post_name, sort, status, remark) VALUES ('ceo', '董事长', 1, 1, '');
INSERT INTO sys_post (post_code, post_name, sort, status, remark) VALUES ('se', '项目经理', 2, 1, '');
INSERT INTO sys_post (post_code, post_name, sort, status, remark) VALUES ('hr', '人力资源', 3, 1, '');
INSERT INTO sys_post (post_code, post_name, sort, status, remark) VALUES ('user', '普通员工', 1, 1, '');
INSERT INTO sys_notice (notice_title, notice_type, notice_content, status) VALUES ('测试通知1', 1, '这是一条测试通知内容', 1);
INSERT INTO sys_notice (notice_title, notice_type, notice_content, status) VALUES ('测试公告2', 2, '这是一条测试公告内容', 1);
INSERT INTO sys_dict_type (dict_name, dict_type, status, remark) VALUES ('用户性别', 'sys_user_sex', 1, '用户性别列表');
INSERT INTO sys_dict_type (dict_name, dict_type, status, remark) VALUES ('通知类型', 'sys_notice_type', 1, '通知类型列表');
INSERT INTO sys_dict_data (dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, remark) VALUES (1, '男', '0', 'sys_user_sex', '1', '1', 'N', 1, '性别男');
INSERT INTO sys_dict_data (dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, remark) VALUES (2, '女', '1', 'sys_user_sex', '1', '1', 'N', 1, '性别女');
INSERT INTO sys_dict_data (dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, remark) VALUES (3, '未知', '2', 'sys_user_sex', '1', '1', 'N', 1, '性别未知');
INSERT INTO sys_dict_data (dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, remark) VALUES (1, '通知', '1', 'sys_notice_type', '1', '1', 'N', 1, '通知');
INSERT INTO sys_dict_data (dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, remark) VALUES (2, '公告', '2', 'sys_notice_type', '1', '1', 'N', 1, '公告');
INSERT INTO sys_dept (parent_id, ancestors, dept_name, sort, leader, phone, email, status, del_flag) VALUES (0, '0', '测试科技', 1, 'admin', '18613030352', '1002219331@qq.com', 1, 1);
INSERT INTO sys_dept (parent_id, ancestors, dept_name, sort, leader, phone, email, status, del_flag) VALUES (1, '0,1', '深圳总公司', 1, '1', '1', 'xx@qq.com', 1, 1);
INSERT INTO sys_dept (parent_id, ancestors, dept_name, sort, leader, phone, email, status, del_flag) VALUES (1, '0,1', '长沙分公司', 2, '1', '1', 'xx@qq.com', 1, 1);
INSERT INTO sys_dept (parent_id, ancestors, dept_name, sort, leader, phone, email, status, del_flag) VALUES (2, '0,1,2', '研发部门', 1, '1', '1', 'xx@qq.com', 1, 1);
INSERT INTO sys_dept (parent_id, ancestors, dept_name, sort, leader, phone, email, status, del_flag) VALUES (2, '0,1,2', '市场部门', 2, '1', '1', 'xx@qq.com', 1, 1);
INSERT INTO sys_dept (parent_id, ancestors, dept_name, sort, leader, phone, email, status, del_flag) VALUES (2, '0,1,2', '测试部门', 3, '1', '1', 'xx@qq.com', 1, 1);
INSERT INTO sys_dept (parent_id, ancestors, dept_name, sort, leader, phone, email, status, del_flag) VALUES (2, '0,1,2', '财务部门', 4, '1', '1', 'xx@qq.com', 1, 1);
INSERT INTO sys_dept (parent_id, ancestors, dept_name, sort, leader, phone, email, status, del_flag) VALUES (2, '0,1,2', '运维部门', 5, '1', '1', 'xx@qq.com', 1, 1);
INSERT INTO sys_dept (parent_id, ancestors, dept_name, sort, leader, phone, email, status, del_flag) VALUES (3, '0,1,3', '市场部门1', 6, '1', '1', 'xx@qq.com', 1, 1);
INSERT INTO sys_dept (parent_id, ancestors, dept_name, sort, leader, phone, email, status, del_flag) VALUES (3, '0,1,3', '财务部门1', 1, '1', '1', 'xx@qq.com', 1, 1);
INSERT INTO sys_menu (id, menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES (1, '首页', 1, 1, 1, 0, '/home', '', 'DashboardOutlined', '首页');
INSERT INTO sys_menu (id, menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES (2, '权限管理', 1, 1, 2, 0, '/system', '', 'SettingOutlined', '权限管理');
INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('用户管理', 2, 1, 1, 2, '/system/user', '', 'UserOutlined', '用户信息管理');
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
INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('角色管理', 2, 1, 2, 2, '/system/role', '', 'UsergroupAddOutlined', '角色信息管理');
INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('添加角色信息', 3, 1, 1, 15, '', '/api/system/role/addRole', '', '添加角色信息');
INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('删除角色信息', 3, 1, 2, 15, '', '/api/system/role/deleteRole', '', '删除角色信息');
INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('更新角色信息', 3, 1, 3, 15, '', '/api/system/role/updateRole', '', '更新角色信息');
INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('更新角色信息状态', 3, 1, 4, 15, '', '/api/system/role/updateRoleStatus', '', '更新角色信息状态');
INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('查询角色信息详情', 3, 1, 5, 15, '', '/api/system/role/queryRoleDetail', '', '查询角色信息详情');
INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('查询角色信息列表', 3, 1, 6, 15, '', '/api/system/role/queryRoleList', '', '查询角色信息列表');
INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('查询角色菜单列表', 3, 1, 7, 15, '', '/api/system/role/queryRoleMenu', '', '查询角色菜单列表');
INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('更新角色菜单信息', 3, 1, 8, 15, '', '/api/system/role/updateRoleMenu', '', '更新角色菜单信息');
INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('查询已分配用户角色', 3, 1, 8, 15, '', '/api/system/role/queryAllocatedList', '', '查询已分配用户角色');
INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('查询未分配用户角色', 3, 1, 8, 15, '', '/api/system/role/queryUnallocatedList', '', '查询未分配用户角色');
INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('取消授权用户', 3, 1, 8, 15, '', '/api/system/role/cancelAuthUser', '', '取消授权用户');
INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('批量取消授权用户', 3, 1, 8, 15, '', '/api/system/role/batchCancelAuthUser', '', '批量取消授权用户');
INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('批量选择用户授权', 3, 1, 8, 15, '', '/api/system/role/batchAuthUser', '', '批量选择用户授权');
INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('菜单管理', 2, 1, 3, 2, '/system/menu', '', 'MenuOutlined', '菜单信息管理');
INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('添加菜单', 3, 1, 1, 29, '', '/api/system/menu/addMenu', '', '添加菜单');
INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('删除菜单', 3, 1, 2, 29, '', '/api/system/menu/deleteMenu', '', '删除菜单');
INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('更新菜单', 3, 1, 3, 29, '', '/api/system/menu/updateMenu', '', '更新菜单');
INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('更新菜单状态', 3, 1, 4, 29, '', '/api/system/menu/updateMenuStatus', '', '更新菜单状态');
INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('查询菜单详情', 3, 1, 5, 29, '', '/api/system/menu/queryMenuDetail', '', '查询菜单详情');
INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('查询菜单列表', 3, 1, 6, 29, '', '/api/system/menu/queryMenuList', '', '查询菜单列表');
INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('查询菜单树', 3, 1, 6, 29, '', '/api/system/menu/queryMenuListSimple', '', '查询菜单树');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('部门管理', 2, 1, 4, 2, '/system/dept', '', 'ApartmentOutlined', '部门管理');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('添加部门', 3, 1, 1, 37, '', '/api/system/dept/addDept', '', '添加部门');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('删除部门', 3, 1, 2, 37, '', '/api/system/dept/deleteDept', '', '删除部门');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('更新部门', 3, 1, 3, 37, '', '/api/system/dept/updateDept', '', '更新部门');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('更新部门状态', 3, 1, 4, 37, '', '/api/system/dept/updateDeptStatus', '', '更新部门状态');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('查询部门详情', 3, 1, 5, 37, '', '/api/system/dept/queryDeptDetail', '', '查询部门详情');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('查询部门列', 3, 1, 6, 37, '', '/api/system/dept/queryDeptList', '', '查询部门列');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('岗位管理', 2, 1, 5, 2, '/system/post', '', 'AuditOutlined', '岗位管理');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('添加岗位', 3, 1, 1, 44, '', '/api/system/post/addPost', '', '添加岗位');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('删除岗位', 3, 1, 2, 44, '', '/api/system/post/deletePost', '', '删除岗位');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('更新岗位', 3, 1, 3, 44, '', '/api/system/post/updatePost', '', '更新岗位');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('更新岗位状态', 3, 1, 4, 44, '', '/api/system/post/updatePostStatus', '', '更新岗位状态');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('查询岗位详情', 3, 1, 5, 44, '', '/api/system/post/queryPostDetail', '', '查询岗位详情');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('查询岗位列', 3, 1, 6, 44, '', '/api/system/post/queryPostList', '', '查询岗位列');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('字典类型', 2, 1, 6, 2, '/system/dictType', '', 'TableOutlined', '字典类型管理');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('添加字典类型', 3, 1, 1, 51, '', '/api/system/dictType/addDictType', '', '添加字典类型');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('删除字典类型', 3, 1, 2, 51, '', '/api/system/dictType/deleteDictType', '', '删除字典类型');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('更新字典类型', 3, 1, 3, 51, '', '/api/system/dictType/updateDictType', '', '更新字典类型');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('更新字典类型状态', 3, 1, 4, 51, '', '/api/system/dictType/updateDictTypeStatus', '', '更新字典类型状态');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('查询字典类型详情', 3, 1, 5, 51, '', '/api/system/dictType/queryDictTypeDetail', '', '查询字典类型详情');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('查询字典类型列', 3, 1, 6, 51, '', '/api/system/dictType/queryDictTypeList', '', '查询字典类型列');
INSERT INTO sys_menu (menu_name, menu_type, status, visible, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('字典数据', 2, 1, 0,7, 2, '/system/dictData', '', 'UngroupOutlined', '字典数据管理');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('添加字典数据', 3, 1, 1, 58, '', '/api/system/dictData/addDictData', '', '添加字典数据');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('删除字典数据', 3, 1, 2, 58, '', '/api/system/dictData/deleteDictData', '', '删除字典数据');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('更新字典数据', 3, 1, 3, 58, '', '/api/system/dictData/updateDictData', '', '更新字典数据');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('更新字典数据状态', 3, 1, 4, 58, '', '/api/system/dictData/updateDictDataStatus', '', '更新字典数据状态');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('查询字典数据详情', 3, 1, 5, 58, '', '/api/system/dictData/queryDictDataDetail', '', '查询字典数据详情');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('查询字典数据列', 3, 1, 6, 58, '', '/api/system/dictData/queryDictDataList', '', '查询字典数据列');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('通知公告', 2, 1, 8, 2, '/system/notice', '', 'MessageOutlined', '通知公告管理');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('添加通知公告', 3, 1, 1, 65, '', '/api/system/notice/addNotice', '', '添加通知公告');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('删除通知公告', 3, 1, 2, 65, '', '/api/system/notice/deleteNotice', '', '删除通知公告');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('更新通知公告', 3, 1, 3, 65, '', '/api/system/notice/updateNotice', '', '更新通知公告');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('更新通知公告状态', 3, 1, 4, 65, '', '/api/system/notice/updateNoticeStatus', '', '更新通知公告状态');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('查询通知公告详情', 3, 1, 5, 65, '', '/api/system/notice/queryNoticeDetail', '', '查询通知公告详情');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('查询通知公告列', 3, 1, 6, 65, '', '/api/system/notice/queryNoticeList', '', '查询通知公告列');
INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('日志管理', 1, 1, 2, 0, '/log', '', 'FilterOutlined', '日志管理');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('登录日志', 2, 1, 9, 72, '/log/loginLog', '', 'DeleteOutlined', '系统访问记录管理');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('添加系统访问记录', 3, 1, 1, 73, '', '/api/system/loginLog/addLoginLog', '', '添加系统访问记录');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('删除系统访问记录', 3, 1, 2, 73, '', '/api/system/loginLog/deleteLoginLog', '', '删除系统访问记录');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('清空系统登录日志', 3, 1, 3, 73, '', '/api/system/loginLog/cleanLoginLog', '', '清空系统登录日志');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('更新系统访问记录状态', 3, 1, 4, 73, '', '/api/system/loginLog/updateLoginLogStatus', '', '更新系统访问记录状态');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('查询系统访问记录详情', 3, 1, 5, 73, '', '/api/system/loginLog/queryLoginLogDetail', '', '查询系统访问记录详情');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('查询系统访问记录列', 3, 1, 6, 73, '', '/api/system/loginLog/queryLoginLogList', '', '查询系统访问记录列');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('操作日志', 2, 1, 10, 72, '/log/operateLog', '', 'ClearOutlined', '操作日志记录管理');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('添加操作日志记录', 3, 1, 1, 80, '', '/api/system/operateLog/addOperateLog', '', '添加操作日志记录');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('删除操作日志记录', 3, 1, 2, 80, '', '/api/system/operateLog/deleteOperateLog', '', '删除操作日志记录');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('清空操作日志记录', 3, 1, 3, 80, '', '/api/system/operateLog/cleanOperateLog', '', '清空操作日志记录');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('更新操作日志记录状态', 3, 1, 4, 80, '', '/api/system/operateLog/updateOperateLogStatus', '', '更新操作日志记录状态');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('查询操作日志记录详情', 3, 1, 5, 80, '', '/api/system/operateLog/queryOperateLogDetail', '', '查询操作日志记录详情');
INSERT INTO sys_menu (menu_name, menu_type, status, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('查询操作日志记录列', 3, 1, 6, 80, '', '/api/system/operateLog/queryOperateLogList', '', '查询操作日志记录列');
INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('其他', 1, 1, 3, 0, '/other', '', 'AudioOutlined', '其他');
INSERT INTO sys_menu (menu_name, menu_type, `status`, sort, parent_id, menu_url, api_url, menu_icon, remark) VALUES ('查询菜单资源', 3, 1, 7, 29, '', '/api/system/menu/queryMenuResourceList', '', '查询菜单资源');



",
        vec![],
    )
        .await
        .unwrap();
    info!("初始化菜单数据");
}
