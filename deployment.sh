#!/bin/bash

cargo build -r

#停止服务
docker stop axum-admin


#删除容器
docker rm -f axum-admin

#删除镜像
docker rmi -f axum-admin:v1

#删除none镜像
docker rmi -f $(docker images | grep "none" | awk '{print $3}')

#构建服务
docker build -t axum-admin:v1 -f Dockerfile .

#启动服务
docker run -itd --net=host --name=axum-admin axum-admin:v1
