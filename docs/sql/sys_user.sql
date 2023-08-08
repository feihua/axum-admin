create table sys_user
(
    id          bigint auto_increment comment '主键'
        primary key,
    mobile      char(11) default ''                not null comment '手机',
    user_name   varchar(50)                        not null comment '姓名',
    password    varchar(64) charset utf8mb3        null comment '密码',
    status_id   tinyint  default 1                 not null comment '状态(1:正常，0:禁用)',
    sort        int      default 1                 not null comment '排序',
    remark      varchar(255)                       null comment '备注',
    create_time datetime default CURRENT_TIMESTAMP not null comment '创建时间',
    update_time datetime default CURRENT_TIMESTAMP not null on update CURRENT_TIMESTAMP comment '修改时间',
    constraint AK_phone
        unique (mobile)
)
    comment '用户信息';

INSERT INTO sys_user (id, mobile, user_name, password, status_id, sort, remark, create_time, update_time) VALUES (1, '18613030352', '超级管理员', '123456', 1, 1, '超级管理员', '2018-03-31 11:22:37', '2022-08-04 23:38:34');
INSERT INTO sys_user (id, mobile, user_name, password, status_id, sort, remark, create_time, update_time) VALUES (2, '13800000000', '普通用户', '123456', 1, 1, '演示权限', '2018-12-28 16:57:47', '2018-12-28 16:57:47');
INSERT INTO sys_user (id, mobile, user_name, password, status_id, sort, remark, create_time, update_time) VALUES (3, '18613030353', 'koobe', '123456', 1, 1, '测试', '2022-07-14 13:54:15', '2022-07-14 13:54:15');
INSERT INTO sys_user (id, mobile, user_name, password, status_id, sort, remark, create_time, update_time) VALUES (4, '18613030350', '11', '123456', 1, 1, '11', '2022-07-14 13:56:54', '2022-07-14 13:56:54');
INSERT INTO sys_user (id, mobile, user_name, password, status_id, sort, remark, create_time, update_time) VALUES (5, '18613030341', '22334', '123456', 0, 1, '22334', '2022-07-14 13:58:20', '2022-07-15 16:13:38');
INSERT INTO sys_user (id, mobile, user_name, password, status_id, sort, remark, create_time, update_time) VALUES (11, '18613030452', '1234', '123456', 0, 1, '1233', '2022-07-14 15:18:12', '2022-07-19 09:14:36');
INSERT INTO sys_user (id, mobile, user_name, password, status_id, sort, remark, create_time, update_time) VALUES (12, '18615200352', 'test', '123456', 1, 1, 'test', '2022-07-26 16:00:08', '2022-07-26 16:00:08');
INSERT INTO sys_user (id, mobile, user_name, password, status_id, sort, remark, create_time, update_time) VALUES (13, '18615050520', '2', '11', 1, 12, '123', '2022-07-26 16:06:07', '2022-07-26 20:21:34');
