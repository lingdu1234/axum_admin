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
/*Data for the table `sys_job` */

insert  into `sys_job`(`job_id`,`task_id`,`task_count`,`run_count`,`job_name`,`job_params`,`job_group`,`invoke_target`,`cron_expression`,`misfire_policy`,`concurrent`,`status`,`create_by`,`update_by`,`remark`,`last_time`,`next_time`,`end_time`,`created_at`,`updated_at`,`deleted_at`) values 
('00UP55D2GGFQ1EDVLRMUIOIU37',1001,500,23,'无参数测试',NULL,'DEFAULT','test_a','0/3 * * * * ?','1','1','0','00TV87DDOBJPU75J4TGUOC3NNG','00TV87DDOBJPU75J4TGUOC3NNG','任务id:1644942763    开始时间:2022-02-15 16:32:43\n任务删除:--------    删除时间:2022-02-15 16:33:53.958541100\n最终运行次数:23','2022-02-15 16:33:52','2022-02-15 16:33:54','2022-02-04 22:49:06','2022-02-04 12:44:31','2022-02-15 11:39:21',NULL),
('00UP5ICG2DC35UMSG4MQ172DBO',1002,50,50,'简单参数测试','简单尝试测试-9988','DEFAULT','test_b','0/1 * * * * ?','3','1','0','00TV87DDOBJPU75J4TGUOC3NNG','00TV87DDOBJPU75J4TGUOC3NNG','任务id:1644672577    开始时间:2022-02-12 13:29:37\n任务更新:--------    更新时间:2022-02-12 13:29:48\n任务名称:简单参数测试    修改后次数:0\n任务时间:0/1 * * * * ?    已运行次数:9\n任务更新:--------    更新时间:2022-02-12 13:29:56\n任务名称:简单参数测试    修改后次数:50\n任务时间:0/1 * * * * ?    已运行次数:17\n任务完毕:--------    完成时间:2022-02-12 13:30:29.516355600\n最终运行次数:50','2022-02-12 13:30:29','2022-02-12 13:30:30','2022-02-04 22:33:32','2022-02-04 12:58:42','2022-02-12 13:29:56',NULL),
('00UP5SCL42G6ECK1M7K68FSD41',1003,0,47,'复杂参数测试','{\"a\":\"复杂参数测试\"}','DEFAULT','test_c','0/1 * * * * ?','3','1','0','00TV87DDOBJPU75J4TGUOC3NNG','00TV87DDOBJPU75J4TGUOC3NNG','任务id:1645909547    开始时间:2022-02-26 21:05:47\n任务删除:--------    删除时间:2022-02-26 21:06:35.940209500\n最终运行次数:47','2022-02-26 21:06:35','2022-02-26 21:06:36','2022-02-04 22:33:39','2022-02-04 13:09:37','2022-02-12 13:30:50',NULL),
('00UQNFB3B5CRKG99Q2NLQDRKHH',1000,0,9,'在线用户检测',NULL,'SYSTEM','check_user_online','0/5 * * * * ?','1','1','0','00TV87DDOBJPU75J4TGUOC3NNG','00TV87DDOBJPU75J4TGUOC3NNG','任务id:1645898889    开始时间:2022-02-26 18:08:09\n任务删除:--------    删除时间:2022-02-26 18:08:52.241087900\n最终运行次数:9','2022-02-26 18:08:51','2022-02-26 18:08:55',NULL,'2022-02-05 18:03:00','2022-02-12 09:56:38',NULL);

/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS */;
/*!40111 SET SQL_NOTES=@OLD_SQL_NOTES */;
