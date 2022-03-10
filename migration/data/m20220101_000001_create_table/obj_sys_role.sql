/*
SQLyog Ultimate
MySQL - 10.6.5-MariaDB-1:10.6.5+maria~focal : Database - wk3
*********************************************************************
*/

/*!40101 SET NAMES utf8 */;

/*!40101 SET SQL_MODE=''*/;

/*!40014 SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0 */;
/*!40101 SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='NO_AUTO_VALUE_ON_ZERO' */;
/*!40111 SET @OLD_SQL_NOTES=@@SQL_NOTES, SQL_NOTES=0 */;
/*Data for the table `sys_role` */

insert  into `sys_role`(`role_id`,`role_name`,`role_key`,`list_order`,`data_scope`,`status`,`remark`,`created_at`,`updated_at`) values 
('00UHIKGRA7JVIEU25NNGI8KTJU','管理员','admin',1,'1','1','admin','2022-01-29 16:07:10','2022-03-07 10:20:58'),
('00UHIKGRA7JVIF025NNH39CPMT','普通用户','user',2,'4','1','普通用户','2022-01-29 16:07:10','2022-03-07 10:21:03'),
('00UHIKGRA7JVIF225NNJ4OH4Q4','lingdu专用','lingdu',2,'4','1','lingdu专用','2022-01-29 16:07:10','2022-03-07 10:21:41'),
('00UHIKGRA7JVIF425NNGE80K1B','Browser','browser',4,'1','1','Browser','2022-01-29 16:07:10','2022-03-07 10:21:35'),
('00UHKP89CT1NDVFN6Q0E8LO7NT','超级管理员','SuperAdmin',0,'1','1','超级管理员','2022-01-29 16:42:38','2022-03-07 10:20:52'),
('00VTAOT7820JCROC5CSKO4PVT5','本部门权限','self-x',101,'3','1','','2022-03-04 15:05:54','2022-03-07 10:21:30'),
('00VTAPCFV0MIB252HAA89LBD46','自己的权限','self-y',102,'5','1','','2022-03-04 15:06:25','2022-03-07 10:21:26'),
('00VTAQD9Q3RFU5HT172L8SMDVF','本部门及以下','slef-z',103,'4','1','','2022-03-04 15:07:32','2022-03-07 10:21:22'),
('00VTAQSSFRU2961VV33V1C6R9R','自定义权限','self-a',104,'2','1','','2022-03-04 15:08:04','2022-03-07 10:21:17');

/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS */;
/*!40111 SET SQL_NOTES=@OLD_SQL_NOTES */;
