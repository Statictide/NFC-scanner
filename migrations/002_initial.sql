
create table entity (
    id integer primary key,
    user_id integer not null references user(id),
    tag_uid varchar(20) unique not null, -- Max 8 bytes?
    name varchar(255) not null,
    parrent_id integer null references entity(id)
);
