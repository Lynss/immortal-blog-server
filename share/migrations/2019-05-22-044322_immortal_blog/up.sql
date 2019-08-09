-- Your SQL goes here
create table if not exists blogs
(
    id         serial primary key,
    data       jsonb     not null,
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp,
    created_by varchar   not null default 'system',
    updated_by varchar   not null default 'system'
);

create table if not exists immortal_users
(
    id         serial primary key,
    nickname   varchar   not null,
    password   varchar   not null,
--  这里设计有问题，简单的系统应该只用单个的角色就好了。。。
    roles      int[]     not null default array [2],
    email      varchar   not null,
    phone      varchar,
    sex        int       not null default 2,
    activated  boolean   not null default false,
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp,
    avatar     varchar   not null default ''
);
comment on column immortal_users.sex is '0->male 1->female 2->unknown';
create unique index immortal_users_nickname_uindex
    on immortal_users (nickname);
create unique index immortal_users_email_uindex
    on immortal_users (email);
create index immortal_users_roles_index
    on immortal_users (roles);
create index immortal_users_created_at_index
    on immortal_users (created_at);
create index immortal_users_updated_at_index
    on immortal_users (updated_at);
insert into immortal_users (nickname, password, roles, email, phone, sex,activated)
values
       ('lynss', 'lynss', '{5}', 'ly1169134156@163.com', '17764189136', 0,true),
       ('immortal', 'immortal', '{5}', 'immortal@163.com', '', 2,false),
       ('brahmin', 'brahmin', '{4}', 'brahmin@163.com', '', 2,false),
       ('kshatriya', 'kshatriya', '{3}', 'kshatriya@163.com', '', 2,false),
       ('vaishya', 'vaishya', '{2}', 'vaishya@163.com', '', 2,false),
       ('untouchable', 'untouchable', '{1}', 'untouchable@163.com', '', 2,false);

create table if not exists roles
(
    id         serial primary key,
    name       varchar   not null,
    status     int       not null default 1,
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp
);
create unique index role_name_uindex on roles (name);
create index roles_created_at_index
    on roles (created_at);
create index roles_updated_at_index
    on roles (updated_at);
create index roles_status_index on roles (status);
comment on column roles.status is '0 for disabled,1 for enabled';

-- initial roles
insert into roles (name)
values ('untouchable'),
       ('vaishya'),
       ('kshatriya'),
       ('brahmin'),
       ('immortal');

create table if not exists permissions
(
    id         serial primary key,
    name       varchar   not null,
    status     int       not null default 1,
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp
);
create unique index permissions_name_uindex on permissions (name);
create index permissions_created_at_index
    on permissions (created_at);
create index permissions_updated_at_index
    on permissions (updated_at);
create index permissions_status_index
    on permissions (status);
insert into permissions (name)
values ('all');
comment on column permissions.status is '0 for disabled,1 for enabled';
insert into permissions (name, status)
values
       ('tag',1),
       ('user',1),
       ('category',1),
       ('role',1);

create table if not exists role_permissions
(
    id            serial primary key,
    role_id       int       not null,
    permission_id int       not null,
    level         int       not null default 2,
    created_at    timestamp not null default current_timestamp,
    updated_at    timestamp not null default current_timestamp
);
create index role_permissions_role_id_index on role_permissions (role_id);
create index role_permissions_permission_id_index on role_permissions (permission_id);
create index role_permissions_level_index on role_permissions (level);
create index role_permissions_created_at_index
    on role_permissions (created_at);
create index role_permissions_updated_at_index
    on role_permissions (updated_at);
comment on column role_permissions.permission_id is '0 is considered * means a role has same level on all permissions';
insert into role_permissions (role_id, permission_id, level)
values (1, 1, 1),
       (2, 1, 2),
       (3, 1, 3),
       (4, 1, 4),
       (5, 1, 5);

create table if not exists tags
(
    id         serial primary key,
    name       varchar   not null,
    color      varchar   not null,
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp,
    created_by varchar   not null default 'system',
    updated_by varchar   not null default 'system'
);
create unique index tags_name_uindex on tags (name);
create index tags_created_at_index
    on tags (created_at);
create index tags_updated_at_index
    on tags (updated_at);
create index tags_created_by_index
    on tags (created_by);
create index tags_updated_by_index
    on tags (updated_by);

create table if not exists categories
(
    id          serial primary key,
    name        varchar   not null,
    description varchar,
    created_at  timestamp not null default current_timestamp,
    updated_at  timestamp not null default current_timestamp,
    created_by  varchar   not null default 'system',
    updated_by  varchar   not null default 'system'
);
create unique index categories_name_uindex on categories (name);
create index categories_created_at_index
    on categories (created_at);
create index categories_updated_at_index
    on categories (updated_at);
create index categories_created_by_index
    on categories (created_by);
create index categories_updated_by_index
    on categories (updated_by);

-- create trigger function
create or replace function trigger_set_timestamp()
    returns trigger as
$$
begin
    new.updated_at = now();
    return new;
end;
$$ language plpgsql;

-- add trigger for all tables on update operation
do $$
    declare
        t record;
    begin
        for t in
            select *
            from information_schema.columns
            where column_name = 'updated_at'
            loop
                execute format('create trigger set_timestamp
                        before update on %I
                        for each row execute procedure trigger_set_timestamp()',
                               t.table_name);
            end loop;
    end;
    $$ language plpgsql;
