
# axum-admin

是基于axum和rbatis的rbac权限管理系统

# 预览地址



# 前端项目

```
https://github.com/feihua/antd-admin.git
```


# 本地启动

```shell
git clone https://github.com/feihua/axum-admin.git
cd axum-admin

修改config.toml配置
[db]
url = "mysql://root:123456@127.0.0.1:3306/axum"
[redis]
url = "redis://:123456@127.0.0.1:6379"

启动 cargo run 

```

# 系统截图

## 用户界面

![user](docs/images/user.jpg)

## 角色分配界面

![user-role](docs/images/user_role.jpg)

## 角色界面

![role](docs/images/role.jpg)

## 角色用户界面

![role](docs/images/role_user.jpg)

## 菜单分配界面

![role-menu](docs/images/role_menu.jpg)

## 菜单界面

![menu](docs/images/menu.jpg)

## 部门界面

![menu](docs/images/dept.jpg)

## 岗位界面

![menu](docs/images/post.jpg)

## 字典界面

![menu](docs/images/dict.jpg)

## 字典数字界面

![menu](docs/images/dict_data.jpg)

## 通知界面

![menu](docs/images/notice.jpg)

## 登录日志

![menu](docs/images/login_log.jpg)