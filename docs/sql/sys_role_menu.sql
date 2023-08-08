create table sys_role_menu
(
    id          bigint auto_increment comment '主键'
        primary key,
    role_id     bigint                             not null comment '角色ID',
    menu_id     bigint                             not null comment '菜单ID',
    status_id   tinyint  default 1                 not null comment '状态(1:正常，0:禁用)',
    sort        int      default 1                 not null comment '排序',
    create_time datetime default CURRENT_TIMESTAMP not null comment '创建时间',
    update_time datetime default CURRENT_TIMESTAMP not null on update CURRENT_TIMESTAMP comment '修改时间'
)
    comment '菜单角色关联表';

INSERT INTO sys_role_menu (id, role_id, menu_id, status_id, sort, create_time, update_time) VALUES (1544492801968857203, 3, 1, 1, 1, '2022-07-22 17:48:01', '2022-07-22 17:48:01');
INSERT INTO sys_role_menu (id, role_id, menu_id, status_id, sort, create_time, update_time) VALUES (1544492801968857204, 3, 3, 1, 1, '2022-07-22 17:48:01', '2022-07-22 17:48:01');
INSERT INTO sys_role_menu (id, role_id, menu_id, status_id, sort, create_time, update_time) VALUES (1544492801968857205, 3, 6, 1, 1, '2022-07-22 17:48:01', '2022-07-22 17:48:01');
INSERT INTO sys_role_menu (id, role_id, menu_id, status_id, sort, create_time, update_time) VALUES (1544492801968857206, 3, 7, 1, 1, '2022-07-22 17:48:01', '2022-07-22 17:48:01');
INSERT INTO sys_role_menu (id, role_id, menu_id, status_id, sort, create_time, update_time) VALUES (1544492801968857207, 3, 8, 1, 1, '2022-07-22 17:48:01', '2022-07-22 17:48:01');
INSERT INTO sys_role_menu (id, role_id, menu_id, status_id, sort, create_time, update_time) VALUES (1544492801968857208, 3, 9, 1, 1, '2022-07-22 17:48:01', '2022-07-22 17:48:01');
INSERT INTO sys_role_menu (id, role_id, menu_id, status_id, sort, create_time, update_time) VALUES (1544492801968857209, 3, 10, 1, 1, '2022-07-22 17:48:01', '2022-07-22 17:48:01');
INSERT INTO sys_role_menu (id, role_id, menu_id, status_id, sort, create_time, update_time) VALUES (1544492801968857210, 3, 11, 1, 1, '2022-07-22 17:48:01', '2022-07-22 17:48:01');
INSERT INTO sys_role_menu (id, role_id, menu_id, status_id, sort, create_time, update_time) VALUES (1544492801968857211, 3, 12, 1, 1, '2022-07-22 17:48:01', '2022-07-22 17:48:01');
INSERT INTO sys_role_menu (id, role_id, menu_id, status_id, sort, create_time, update_time) VALUES (1544492801968857212, 3, 13, 1, 1, '2022-07-22 17:48:01', '2022-07-22 17:48:01');
INSERT INTO sys_role_menu (id, role_id, menu_id, status_id, sort, create_time, update_time) VALUES (1544492801968857213, 3, 14, 1, 1, '2022-07-22 17:48:01', '2022-07-22 17:48:01');
INSERT INTO sys_role_menu (id, role_id, menu_id, status_id, sort, create_time, update_time) VALUES (1544492801968857214, 3, 15, 1, 1, '2022-07-22 17:48:01', '2022-07-22 17:48:01');
INSERT INTO sys_role_menu (id, role_id, menu_id, status_id, sort, create_time, update_time) VALUES (1544492801968857215, 3, 16, 1, 1, '2022-07-22 17:48:01', '2022-07-22 17:48:01');
INSERT INTO sys_role_menu (id, role_id, menu_id, status_id, sort, create_time, update_time) VALUES (1544492801968857216, 3, 31, 1, 1, '2022-07-22 17:48:01', '2022-07-22 17:48:01');
INSERT INTO sys_role_menu (id, role_id, menu_id, status_id, sort, create_time, update_time) VALUES (1544492801968857217, 3, 30, 1, 1, '2022-07-22 17:48:01', '2022-07-22 17:48:01');
INSERT INTO sys_role_menu (id, role_id, menu_id, status_id, sort, create_time, update_time) VALUES (1544492801968857218, 3, 2, 1, 1, '2022-07-22 17:48:01', '2022-07-22 17:48:01');
INSERT INTO sys_role_menu (id, role_id, menu_id, status_id, sort, create_time, update_time) VALUES (1544492801968857219, 5, 32, 1, 1, '2022-07-26 15:13:29', '2022-07-26 15:13:29');
INSERT INTO sys_role_menu (id, role_id, menu_id, status_id, sort, create_time, update_time) VALUES (1544492801968857220, 5, 1, 1, 1, '2022-07-26 15:13:29', '2022-07-26 15:13:29');
INSERT INTO sys_role_menu (id, role_id, menu_id, status_id, sort, create_time, update_time) VALUES (1544492801968857221, 4, 1, 1, 1, '2022-07-26 23:04:02', '2022-07-26 23:04:02');
INSERT INTO sys_role_menu (id, role_id, menu_id, status_id, sort, create_time, update_time) VALUES (1544492801968857222, 4, 30, 1, 1, '2022-07-26 23:04:02', '2022-07-26 23:04:02');
INSERT INTO sys_role_menu (id, role_id, menu_id, status_id, sort, create_time, update_time) VALUES (1544492801968857223, 4, 32, 1, 1, '2022-07-26 23:04:02', '2022-07-26 23:04:02');
