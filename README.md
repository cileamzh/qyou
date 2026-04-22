# Qyou 简介，易用的问卷平台

 Qyou 是一个Rust+Vue 构建的 问卷平台，旨在提功简洁便利的问卷系统，

 ## 打包
 Qyou 依赖于Rust cargo和Node npm 进行打包，build.js是Qyou提供的一键打包脚本，打包结果将放在根目录下build文件夹下，qyou /qyou.exe 是 二进制文件，static 是Qyou 依赖的web服务页面
 ## 数据结构
 Qyou并未使用数据库，Qyou使用json存储表格以及记录，根目录下的docs目录是Qyou数据的存放地以t.json结尾的是Qyou的问卷表结构, 同表名目录下是对应表对应的填写情况和记录, 记录文件以r.json结尾
 ## 预览

 ### 主页面

![主页面预览](/preview/pv1.png)

### 编辑页

![编辑页预览](/preview/pv2.png) 

### 填表页

![填表页预览](/preview/pv3.png)
