# Rbatis Demo

A branch to show how [rbatis](https://github.com/rbatis/rbatis) is working.

## How to use

1. start and init your mysql database with script `./sql/schema.sql`:

```mysql
USE `test`;

DROP TABLE IF EXISTS `pets`;
CREATE TABLE `pets`
(
    `id`    INT(10) AUTO_INCREMENT NOT NULL COMMENT '宠物编号',
    `name`  VARCHAR(20)            NOT NULL COMMENT '宠物名称',
    `age`   TINYINT(3)             NOT NULL COMMENT '宠物年龄',
    `photo` VARCHAR(30) DEFAULT NULL COMMENT '宠物图片',
    `ctime` DATETIME    DEFAULT CURRENT_TIMESTAMP,
    `utime` DATETIME    DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (`id`)
) ENGINE = InnoDB,
  DEFAULT CHARSET = utf8mb4 COMMENT ='宠物表';

INSERT INTO `pets` (ID, NAME, AGE)
VALUES (1, 'cat', '1');
INSERT INTO `pets` (ID, NAME, AGE)
VALUES (2, 'dog', '2');
INSERT INTO `pets` (ID, NAME, AGE)
VALUES (3, 'mouse', '3');
```

2. change `RB.link("mysql://root:123456@localhost:3306/test"` to user own MySQL config;

3. run test in `./dao/src/pet`;

> change `#[cfg(not(test))]` to `#[cfg(test)]`;
