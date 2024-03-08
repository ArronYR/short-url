CREATE TABLE IF NOT EXISTS `link`
(
    `id`           bigint(20) unsigned NOT NULL AUTO_INCREMENT COMMENT '主键',
    `short_id`     varchar(50)         NOT NULL COMMENT '短链接',
    `original_url` varchar(2048)       NOT NULL COMMENT '源链接',
    `expired_ts`   bigint(20)          NOT NULL DEFAULT '0' COMMENT '过期时间',
    `status`       int(4)              NOT NULL DEFAULT '0' COMMENT '状态：0正常、1禁用',
    `create_time`  datetime            NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uniq_short_url` (`short_id`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE = utf8mb4_bin COMMENT ='链接记录';

CREATE TABLE IF NOT EXISTS `access_log`
(
    `id`          bigint(20) unsigned NOT NULL AUTO_INCREMENT COMMENT '主键',
    `short_id`    varchar(50)         NOT NULL COMMENT '短链ID',
    `req_headers` longtext COMMENT '请求头',
    `create_time` datetime            NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    PRIMARY KEY (`id`),
    KEY `idx_short_id` (`short_id`) USING BTREE
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4 COMMENT ='访问日志';
