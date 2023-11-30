DROP TABLE IF EXISTS `hero`, `team`, `log`;

CREATE TABLE `hero`
(
    `id`      int         NOT NULL AUTO_INCREMENT,
    `name`    varchar(20) NOT NULL COMMENT '英雄名',
    `line`    int         NOT NULL COMMENT '英雄分路',
    `is_pick` tinyint(1) NOT NULL DEFAULT '0' COMMENT '是否被选择',
    PRIMARY KEY (`id`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE `log`
(
    `id`         int                                NOT NULL AUTO_INCREMENT,
    `team_id`    int                                NOT NULL COMMENT '队伍id',
    `pick_group` varchar(100) CHARACTER SET utf8mb4 NOT NULL COMMENT '抽取组合',
    `time`       datetime                           NOT NULL COMMENT '记录时间',
    PRIMARY KEY (`id`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE `team`
(
    `id`           int NOT NULL AUTO_INCREMENT,
    `encrypt_code` varchar(20) CHARACTER SET utf8mb4  DEFAULT NULL COMMENT '队伍秘钥',
    `pick_content` varchar(100) CHARACTER SET utf8mb4 DEFAULT NULL COMMENT '抽取结果',
    `is_picked`    tinyint(1) NOT NULL DEFAULT '0' COMMENT '是否抽取过',
    `update_time`  datetime                           DEFAULT NULL COMMENT '更新时间',
    PRIMARY KEY (`id`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
