create table user (
    id integer primary key,
    name text not null,
    username text not null,
    admin boolean default false
);

create table session (
    id integer primary key,
    user_id integer not null references user(id) on delete cascade,
    token text unique not null
);
