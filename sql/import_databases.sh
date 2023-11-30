#!/bin/bash

# 停止脚本在遇到错误时继续执行
set -e

# 创建数据库（如果不存在）
mysql -u root -p -e "CREATE DATABASE IF NOT EXISTS lottery;"
mysql -u root -p -e "CREATE DATABASE IF NOT EXISTS lottery_test;"

# 导入第一个数据库
mysql -u root -p lottery < ./sql/lottery.sql

# 导入第二个数据库
mysql -u root -p lottery_test < ./sql/lottery_test.sql

echo "两个数据库已成功导入。"
