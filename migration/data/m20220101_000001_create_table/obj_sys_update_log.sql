/*
SQLyog Ultimate v13.1.1 (64 bit)
MySQL - 10.6.4-MariaDB : Database - poem-admin-demo
*********************************************************************
*/

/*!40101 SET NAMES utf8 */;

/*!40101 SET SQL_MODE=''*/;

/*!40014 SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0 */;
/*!40014 SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0 */;
/*!40101 SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='NO_AUTO_VALUE_ON_ZERO' */;
/*!40111 SET @OLD_SQL_NOTES=@@SQL_NOTES, SQL_NOTES=0 */;
/*Data for the table `sys_update_log` */

insert  into `sys_update_log`(`id`,`app_version`,`backend_version`,`title`,`content`,`created_at`,`updated_at`,`deleted_at`,`updated_by`) values 
('0115HS9TCO2DFMG0NKUTSB4OPN','0.0.1','0.0.1','预览版本','### 预览版本\n\n---\n#### 功能定位\n1. 单机后台管理程序，主要用于自己工作中一些数据处理终端，储存一些数据用于分析处理；\n2. 定位于单机程序，未使用redis等缓存程序，但是使用了内存作为缓存，以api为纬度对数据进行缓存，以数据库为中间值将api关联起来，api更新数据时，清除关联api缓存数据\n\n#### 完成功能\n- [x] 用户管理：用户是系统操作者，该功能主要完成系统用户配置。\n\n- [x] 部门管理：配置系统组织机构（公司、部门、小组），树结构展现支持数据权限。\n\n- [x] 岗位管理：配置系统用户所属担任职务。\n\n- [x] 菜单管理：配置系统菜单，操作权限，按钮权限标识等。\n\n- [x] 角色管理：角色菜单权限分配、设置角色按机构进行数据范围权限划分。\n\n- [x] 字典管理：对系统中经常使用的一些较为固定的数据进行维护。\n\n- [x] 登录日志：系统登录日志记录查询包含登录异常。\n\n- [x] 在线用户：当前系统中活跃用户状态监控。\n\n- [x] 定时任务：在线(添加、修改、删除)任务调度包含执行结果日志。\n\n- [x] 角色切换，不同角色可以数据权限不一致。\n\n- [x] 数据权限：分全部权限，本部门权限，本部门及以下权限，自定义权限，本人权限 五种权限\n\n- [x] 部门切换：可以设置用户可以存在的多个部门，但是只能激活一个部门，可以切换；\n\n- [x] 系统监控：系统信息的简单监控；\n\n- [x] 数据缓存：根据api缓存数据,分公共缓存(所有人缓存数据一致，用于公共数据缓存)和个人缓存(同一api不同用户不同数据的api缓存)，通过数据库名称将api关联在一起，当有数据更新时，清除关联api缓存数据，缓存时间到期，缓存数据清除；\n\n- [x] 操作日志：在菜单设置每个api的日志记录级别，分为文件记录，数据库记录，同时记录，不记录几种模式，根据不同api单独配置\n\n- [x] 权限管理: 由后端返回路由动态生成路由；前端按钮级权限统一由后端返回权限标志控制\n\n#### 说明\n暂时该程序还为完成，而且是个人项目，数据库设计，api都可能更改 ','2022-04-04 20:53:31','2022-04-04 20:53:31',NULL,'00TV876BOIIDCR9H7JA1KNNIGH');

/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS */;
/*!40014 SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS */;
/*!40111 SET SQL_NOTES=@OLD_SQL_NOTES */;
