-- Your SQL goes here
create table if not exists blog
(
    id   serial primary key,
    data jsonb not null
);

create table if not exists immortal_user
(
    id   serial primary key ,
    nick_name varchar not null ,
    password varchar not null,
    role text[] not null default array['normal'],
    email varchar,
    phone varchar,
    sex int not null default 2,
    created_at timestamp not null default current_timestamp,
    avatar varchar not null default ''
);
comment on column immortal_user.sex is '0->male 1->female 2->unknown';
create index immortal_user_nick_name_index
    on immortal_user (nick_name);
create index immortal_user_created_at_index
    on immortal_user (created_at desc);

insert into immortal_user (nick_name, password, email, phone, sex)
values ('lynss','lynss','ly1169134156@163.com','17764189136',0);
