create table sys_user
(
    id           bigint unsigned auto_increment comment '主键'
        primary key,
    gmt_create   datetime         default CURRENT_TIMESTAMP not null comment '创建时间',
    gmt_modified datetime         default CURRENT_TIMESTAMP not null comment '修改时间',
    status_id    tinyint unsigned default 1                 not null comment '状态(1:正常，0:禁用)',
    sort         int unsigned     default 1                 not null comment '排序',
    user_no      bigint                                     not null comment '用户编号',
    mobile       char(11)         default ''                not null comment '手机',
    real_name    varchar(50)                                not null comment '真实姓名',
    remark       varchar(255)                               null comment '备注',
    password     varchar(64) charset utf8                   null comment '密码',
    constraint AK_phone
        unique (mobile)
)
    comment '后台用户信息' charset = utf8mb4;

INSERT INTO sys_user (id, gmt_create, gmt_modified, status_id, sort, user_no, mobile, real_name, remark, password) VALUES (1, '2018-03-31 11:22:37', '2022-08-04 23:38:34', 1, 1, 2018033111202589416, '18800000000', '超级管理员', '超级管理员', '123456');
INSERT INTO sys_user (id, gmt_create, gmt_modified, status_id, sort, user_no, mobile, real_name, remark, password) VALUES (2, '2018-12-28 16:57:47', '2018-12-28 16:57:47', 1, 1, 2018112209531803, '13800000000', '普通用户', '演示权限', '123456');
INSERT INTO sys_user (id, gmt_create, gmt_modified, status_id, sort, user_no, mobile, real_name, remark, password) VALUES (3, '2022-07-14 13:54:15', '2022-07-14 13:54:15', 1, 1, 11, '18613030352', 'koobe', '测试', '123456');
INSERT INTO sys_user (id, gmt_create, gmt_modified, status_id, sort, user_no, mobile, real_name, remark, password) VALUES (4, '2022-07-14 13:56:54', '2022-07-14 13:56:54', 1, 1, 11, '18613030350', '11', '11', '123456');
INSERT INTO sys_user (id, gmt_create, gmt_modified, status_id, sort, user_no, mobile, real_name, remark, password) VALUES (5, '2022-07-14 13:58:20', '2022-07-15 16:13:38', 0, 1, 0, '18613030341', '22334', '22334', '123456');
INSERT INTO sys_user (id, gmt_create, gmt_modified, status_id, sort, user_no, mobile, real_name, remark, password) VALUES (11, '2022-07-14 15:18:12', '2022-07-19 09:14:36', 0, 1, 0, '18613030452', '1234', '1233', '123456');
INSERT INTO sys_user (id, gmt_create, gmt_modified, status_id, sort, user_no, mobile, real_name, remark, password) VALUES (12, '2022-07-26 16:00:08', '2022-07-26 16:00:08', 1, 1, 11, '18615200352', 'test', 'test', '123456');
INSERT INTO sys_user (id, gmt_create, gmt_modified, status_id, sort, user_no, mobile, real_name, remark, password) VALUES (13, '2022-07-26 16:06:07', '2022-07-26 20:21:34', 1, 12, 0, '18615050520', '2', '123', '11');
