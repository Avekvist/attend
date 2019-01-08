CREATE TABLE `attendee` (
  `attendee_id` int NOT NULL AUTO_INCREMENT,
  `tag_id` varchar(50) NOT NULL,
  `attendee_name` varchar(100) NOT NULL,
  PRIMARY KEY (`attendee_id`),
  KEY `FK` (`tag_id`)
);

CREATE TABLE `teacherclassbridge` (
  `teacher_class_bridge_id` int NOT NULL AUTO_INCREMENT,
  `username` varchar(20) NOT NULL,
  `class_id` int NOT NULL,
  PRIMARY KEY (`teacher_class_bridge_id`),
  KEY `FK` (`username`, `class_id`)
);

CREATE TABLE `attendeeclassbridge` (
  `attendee_class_bridge_id` int NOT NULL AUTO_INCREMENT,
  `attendee_id` int NOT NULL,
  `class_id` int NOT NULL,
  PRIMARY KEY (`attendee_class_bridge_id`),
  KEY `FK` (`attendee_id`, `class_id`)
);

CREATE TABLE `tag` (
  `tag_id` varchar(50) NOT NULL,
  `tag_date` datetime DEFAULT CURRENT_TIMESTAMP NOT NULL,
  PRIMARY KEY (`tag_id`)
);

CREATE TABLE `attendance` (
  `attendance_id` int NOT NULL AUTO_INCREMENT,
  `class_id` int NOT NULL,
  `attendee_id` int NOT NULL,
  `attendance_date` varchar(20) NOT NULL,
  `attendance_time` varchar(20) NOT NULL,
  PRIMARY KEY (`attendance_id`),
  KEY `FK` (`class_id`, `attendee_id`)
);

CREATE TABLE `teacher` (
  `username` varchar(20) NOT NULL,
  `password` varchar(100) NOT NULL,
  `teacher_name` varchar(100) NOT NULL,
  `is_admin` tinyint(1) NOT NULL,
  PRIMARY KEY (`username`)
);

CREATE TABLE `class` (
  `class_id` int NOT NULL AUTO_INCREMENT,
  `class_name` varchar(20) NOT NULL,
  PRIMARY KEY (`class_id`)
);

