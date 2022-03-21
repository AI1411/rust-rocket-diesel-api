-- Your SQL goes here
create table todo
(
    id      serial primary key,
    title   varchar(256) not null,
    checked boolean      not null default false
);
