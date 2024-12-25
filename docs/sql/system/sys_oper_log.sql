drop table if exists sys_operate_log;
create table sys_operate_log
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

