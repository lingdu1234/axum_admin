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
('00UHIKGRA7JVIEU25NNGI8KTJU','管理员','admin',1,'2','1','admin','2022-01-29 16:07:10',NULL),
('00UHIKGRA7JVIF025NNH39CPMT','普通用户','user',2,'4','1','普通用户','2022-01-29 16:07:10','2022-02-20 17:09:47'),
('00UHIKGRA7JVIF225NNJ4OH4Q4','lingdu专用','lingdu',2,'4','1','lingdu专用','2022-01-29 16:07:10',NULL),
('00UHIKGRA7JVIF425NNGE80K1B','Browser','browser',4,'2','1','Browser','2022-01-29 16:07:10','2022-02-07 21:30:50'),
('00UHKP89CT1NDVFN6Q0E8LO7NT','超级管理员','SuperAdmin',0,'1','1','超级管理员','2022-01-29 16:42:38','2022-02-20 15:45:23');

/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS */;
/*!40111 SET SQL_NOTES=@OLD_SQL_NOTES */;
