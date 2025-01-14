create table users
(
    id integer not null constraint users_pk primary key,
    user_name varchar(32) not null,
    password varchar(256) not null
);

alter table users owner to postgres;

