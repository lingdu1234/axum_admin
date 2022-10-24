<p align="center">
  <img src='data/logo.svg' />
  <br/>
  <span size="30">Poem-Admin</span>
  <br>
   <a href="./README.md"><span>中文</span></a>   |    <a href="./README_EN.md"><span>english</span></a>
</p>

## 简介

一个由 [axum](https://github.com/tokio-rs/axum)/[poem](https://github.com/poem-web/poem),[sea-orm](https://github.com/SeaQL/sea-orm),[vue3](https://github.com/vuejs/core) 编写的 admin 管理面板，

## 前端项目：   

https://github.com/lingdu1234/IUI314

## 预览地址：
[iui314.vercel.app](https://iui314.vercel.app/#/)
[预览 iui314preview](https://iui314preview.iu1314.top/)


## 完成的功能

- [x] 用户管理：用户是系统操作者，该功能主要完成系统用户配置。

- [x] 部门管理：配置系统组织机构（公司、部门、小组），树结构展现支持数据权限。

- [x] 岗位管理：配置系统用户所属担任职务。

- [x] 菜单管理：配置系统菜单，操作权限，按钮权限标识等。

- [x] 角色管理：角色菜单权限分配、设置角色按机构进行数据范围权限划分。

- [x] 字典管理：对系统中经常使用的一些较为固定的数据进行维护。

- [x] 登录日志：系统登录日志记录查询包含登录异常。

- [x] 在线用户：当前系统中活跃用户状态监控。

- [x] 定时任务：在线(添加、修改、删除)任务调度包含执行结果日志。

- [x] 角色切换，不同角色可以数据权限不一致。

- [x] 数据权限：分全部权限，本部门权限，本部门及以下权限，自定义权限，本人权限 五种权限

- [x] 部门切换：可以设置用户可以存在的多个部门，但是只能激活一个部门，可以切换；

- [x] 系统监控：完成系统信息的简单监控；

- [x] 数据缓存：根据api缓存数据,分公共缓存(所有人缓存数据一致，用于公共数据缓存)和个人缓存(同一api不同用户不同数据的api缓存)，通过数据库名称将api关联在一起，当有数据更新时，清除关联api缓存数据，缓存时间到期，缓存数据清除；

- [x] 操作日志：在菜单设置每个api的日志记录级别，分为文件记录，数据库记录，同时记录，不记录几种模式，根据不同api单独配置

- [x] 权限管理: 由后端返回路由动态生成路由；前端按钮级权限统一由后端返回权限标志控制
；

## 说明

1. 部门切换，角色切换，数据权限，部门切换 3者组合可以实现诡异的权限组合


## 数据迁移
1. 安装sea-orm cli 工具 `cargo install sea-orm-cli`
2. 在 .env中设置数据库地址
3. 运行迁移命令：`sea-orm-cli migrate up`

* 数据库迁移重置：`sea-orm-cli migrate fresh`
* 数据库降级：`sea-orm-cli migrate down`
* 更多数据库迁移操作详见：[https://www.sea-ql.org/SeaORM/docs/migration/running-migration](https://www.sea-ql.org/SeaORM/docs/migration/running-migration)
