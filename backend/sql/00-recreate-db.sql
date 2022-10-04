-- DEV ONLY - Brute Force recreate DB for live dev and unit test
-- select pg_terminate_backend(pid) from pg_stat_activity where usename = 'app_user';
DROP DATABASE IF EXISTS pg_demo;
-- DROP USER IF EXISTS app_user;

-- DEV ONLY - for quick iteration
-- CREATE USER app_user PASSWORD 'app_pwd_to_change';
CREATE DATABASE pg_demo --owner app_user ENCODING = 'UTF-8';