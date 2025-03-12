#!/bin/bash

 script
# 编译构建项目，-r 表示进行发布构建
cargo build -r

# 停止名为 axum-admin 的容器
docker stop axum-admin

# 强制移除名为 axum-admin 的容器
docker rm -f axum-admin

# 强制移除标签为 axum-admin:v1 的镜像
docker rmi -f axum-admin:v1

# 移除所有未被标记的镜像
docker rmi -f $(docker images | grep "none" | awk '{print $3}')

# 使用 Dockerfile 构建新的镜像并打上标签 axum-admin:v1
docker build -t axum-admin:v1 -f Dockerfile .

# 以交互模式和守护进程方式运行容器，使用主机网络并命名为 axum-admin
docker run -itd --net=host --name=axum-admin axum-admin:v1

