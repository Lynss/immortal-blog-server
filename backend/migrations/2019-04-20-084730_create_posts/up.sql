-- Your SQL goes here
create table if not exists blog
(
    id         serial primary key,
    data       jsonb     not null,
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp
);

create table if not exists immortal_user
(
    id         serial primary key,
    nickname   varchar   not null,
    password   varchar   not null,
    role       int[]     not null default array [1],
    email      varchar   not null,
    phone      varchar,
    sex        int       not null default 2,
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp,
    avatar     varchar   not null default ''
);
comment on column immortal_user.sex is '0->male 1->female 2->unknown';
create unique index immortal_user_nickname_uindex
    on immortal_user (nickname);
create index immortal_user_created_at_index
    on immortal_user (created_at);
create index immortal_user_updated_at_index
    on immortal_user (updated_at);

insert into immortal_user (id, nickname, password, role, email, phone, sex)
values (0, 'lynss', 'lynss', '{4}', 'ly1169134156@163.com', '17764189136', 0);

create table if not exists role
(
    id         serial primary key,
    name       varchar   not null,
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp
);
create unique index role_name_uindex on role (name);
create index role_created_at_index
    on role (created_at);
create index role_updated_at_index
    on role (updated_at);
-- initial roles
insert into role (id, name)
values (0, 'untouchable'),
       (1, 'vaishya'),
       (2, 'kshatriya'),
       (3, 'brahmin'),
       (4, 'immortal');

create table if not exists permission
(
    id         serial primary key,
    name       varchar   not null,
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp
);
create unique index permission_name_uindex on permission (name);
create index permission_created_at_index
    on permission (created_at);
create index permission_updated_at_index
    on permission (updated_at);

create table if not exists role_permission
(
    id            serial primary key,
    role_id       int       not null,
    permission_id int       not null,
    level         int       not null default 1,
    created_at    timestamp not null default current_timestamp,
    updated_at    timestamp not null default current_timestamp
);
create index role_permission_role_id_index on role_permission (role_id);
create index role_permission_permission_id_index on role_permission (permission_id);
create index role_permission_level_index on role_permission (level);
create index role_permission_created_at_index
    on role_permission (created_at);
create index role_permission_updated_at_index
    on role_permission (updated_at);
comment on column role_permission.permission_id is '-1 is considered * means a role has same level on all permissions';
insert into role_permission (role_id, permission_id, level)
values (0, -1, 0),
       (1, -1, 1),
       (2, -1, 2),
       (3, -1, 3),
       (4, -1, 4);

-- add trigger for all table on update operation
create or replace function trigger_set_timestamp()
    returns trigger as
$$
begin
    new.updated_at = now();
    return new;
end;
$$ language plpgsql;

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
