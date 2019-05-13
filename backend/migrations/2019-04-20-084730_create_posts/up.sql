-- Your SQL goes here
create table if not exists blogs
(
    id         serial primary key,
    data       jsonb     not null,
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp
);

create table if not exists immortal_users
(
    id         serial primary key,
    nickname   varchar   not null,
    password   varchar   not null,
    roles       int[]     not null default array [2],
    email      varchar   not null,
    phone      varchar,
    sex        int       not null default 2,
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp,
    avatar     varchar   not null default ''
);
comment on column immortal_users.sex is '0->male 1->female 2->unknown';
create unique index immortal_user_nickname_uindex
    on immortal_users (nickname);
create index immortal_user_roles_index
    on immortal_users (roles);
create index immortal_user_created_at_index
    on immortal_users (created_at);
create index immortal_user_updated_at_index
    on immortal_users (updated_at);
insert into immortal_users (id, nickname, password, roles, email, phone, sex)
values (1, 'lynss', 'lynss', '{5}', 'ly1169134156@163.com', '17764189136', 0);

create table if not exists roles
(
    id         serial primary key,
    name       varchar   not null,
    status     int not null default 1,
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp
);
create unique index role_name_uindex on roles (name);
create index role_created_at_index
    on roles (created_at);
create index role_updated_at_index
    on roles (updated_at);
create index role_status_index on roles (status);
comment on column roles.status is '0 for disabled,1 for enabled';

-- initial roles
insert into roles (id, name)
values (1, 'untouchable'),
       (2, 'vaishya'),
       (3, 'kshatriya'),
       (4, 'brahmin'),
       (5, 'immortal');

create table if not exists permissions
(
    id         serial primary key,
    name       varchar   not null,
    status     int not null default 1,
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp
);
create unique index permission_name_uindex on permissions (name);
create index permission_created_at_index
    on permissions (created_at);
create index permission_updated_at_index
    on permissions (updated_at);
create index permission_status_index
    on permissions (status);
insert into permissions (id, name)
values (0, 'all');
comment on column permissions.status is '0 for disabled,1 for enabled';


create table if not exists role_permissions
(
    id            serial primary key,
    role_id       int       not null,
    permission_id int       not null,
    level         int       not null default 2,
    created_at    timestamp not null default current_timestamp,
    updated_at    timestamp not null default current_timestamp
);
create index role_permission_role_id_index on role_permissions (role_id);
create index role_permission_permission_id_index on role_permissions (permission_id);
create index role_permission_level_index on role_permissions (level);
create index role_permission_created_at_index
    on role_permissions (created_at);
create index role_permission_updated_at_index
    on role_permissions (updated_at);
comment on column role_permissions.permission_id is '0 is considered * means a role has same level on all permissions';
insert into role_permissions (role_id, permission_id, level)
values (1, 0, 1),
       (2, 0, 2),
       (3, 0, 3),
       (4, 0, 4),
       (5, 0, 5);

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
