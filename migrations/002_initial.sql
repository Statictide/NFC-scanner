
create table collection (
    id integer primary key,
    name varchar(255),
    tag_id varchar(255) unique not null,
    user_id integer not null references user(id)
);

create table entity (
    id integer primary key,
    name varchar(255),
    tag_id varchar(255) unique not null,
    user_id integer not null references user(id),
    collection_id integer null references collection(id)
);
