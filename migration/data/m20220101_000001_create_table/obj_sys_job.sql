/*
SQLyog Ultimate
MySQL - 10.6.5-MariaDB-1:10.6.5+maria~focal : Database - poem_demo
*********************************************************************
*/

/*!40101 SET NAMES utf8 */;

/*!40101 SET SQL_MODE=''*/;

/*!40014 SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0 */;
/*!40101 SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='NO_AUTO_VALUE_ON_ZERO' */;
/*!40111 SET @OLD_SQL_NOTES=@@SQL_NOTES, SQL_NOTES=0 */;
/*Data for the table `sys_job` */

insert  into `sys_job`(`job_id`,`task_id`,`task_count`,`run_count`,`job_name`,`job_params`,`job_group`,`invoke_target`,`cron_expression`,`misfire_policy`,`concurrent`,`status`,`create_by`,`update_by`,`remark`,`last_time`,`next_time`,`end_time`,`created_at`,`updated_at`,`deleted_at`) values 
('00UP55D2GGFQ1EDVLRMUIOIU37',1001,500,17,'无参数测试',NULL,'DEFAULT','test_a','0/3 * * * * ?','1','1','0','00TV87DDOBJPU75J4TGUOC3NNG','00TV87DDOBJPU75J4TGUOC3NNG','任务id:1646775969    开始时间:2022-03-08 21:46:09\n任务删除:--------    删除时间:2022-03-08 21:47:03.759519400\n最终运行次数:17','2022-03-08 21:47:01','2022-03-08 21:47:03','2022-02-04 22:49:06','2022-02-04 12:44:31','2022-02-15 11:39:21',NULL),
('00UP5ICG2DC35UMSG4MQ172DBO',1002,1000,175,'简单参数测试','简单尝试测试-9988','DEFAULT','test_b','0/1 * * * * ?','3','1','0','00TV87DDOBJPU75J4TGUOC3NNG','00TV87DDOBJPU75J4TGUOC3NNG','任务id:1646840964    开始时间:2022-03-09 15:49:24\n任务删除:--------    删除时间:2022-03-09 15:52:20.823642\n最终运行次数:175','2022-03-09 15:52:20','2022-03-09 15:52:21','2022-02-04 22:33:32','2022-02-04 12:58:42','2022-03-08 21:09:01',NULL),
('00UP5SCL42G6ECK1M7K68FSD41',1003,9999,86,'复杂参数测试','{\"a\":\"复杂参数测试aaaaa\"}','DEFAULT','test_c','0/2 * * * * ?','3','1','0','00TV87DDOBJPU75J4TGUOC3NNG','00TV87DDOBJPU75J4TGUOC3NNG','任务id:1646840965    开始时间:2022-03-09 15:49:25\n任务删除:--------    删除时间:2022-03-09 15:52:18.443174200\n最终运行次数:86','2022-03-09 15:52:17','2022-03-09 15:52:18','2022-02-04 22:33:39','2022-02-04 13:09:37','2022-03-08 21:09:07',NULL),
('00UQNFB3B5CRKG99Q2NLQDRKHH',1000,0,19,'在线用户检测',NULL,'SYSTEM','check_user_online','0/30 * * * * ?','1','1','0','00TV87DDOBJPU75J4TGUOC3NNG','00TV87DDOBJPU75J4TGUOC3NNG','任务id:1646837487    开始时间:2022-03-09 14:51:27\n任务删除:--------    删除时间:2022-03-09 15:00:57.957587300\n最终运行次数:19','2022-03-09 15:00:31','2022-03-09 15:01:00',NULL,'2022-02-05 18:03:00','2022-03-06 11:24:38',NULL),
('01010V2BOSKKCVKRVTKTMQ1AKI',2001,0,0,'更新api信息',NULL,'SYSTEM','update_api_info','13 0/5 * * * ?','1','1','0','00TV87DDOBJPU75J4TGUOC3NNG',NULL,'任务id:1646775100    开始时间:2022-03-08 21:31:40\n任务删除:--------    删除时间:2022-03-08 21:31:58.688371\n最终运行次数:0',NULL,'2022-03-08 21:35:13',NULL,'2022-03-07 11:57:02',NULL,NULL);

/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS */;
/*!40111 SET SQL_NOTES=@OLD_SQL_NOTES */;
