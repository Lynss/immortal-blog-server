-- create user
create role immortal with password 'immortal' superuser ;
create database immortal owner immortal;
grant all privileges on database immortal to immortal;