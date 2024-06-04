create table entity (
    id integer primary key,
    tag_id varchar(255) unique not null,
    name varchar(255),
    owner varchar(255),
    created_at timestamp default current_timestamp not null,
    updated_at timestamp default current_timestamp not null
);

create trigger update_entity_updated_at
after update on entity
for each row
begin
    update entity set updated_at = current_timestamp where id = old.id;
end;


create table user (
    id integer primary key,
    name text not null,
    username text not null,
    created_at timestamp default current_timestamp not null,
    updated_at timestamp default current_timestamp not null
);

create trigger update_user_updated_at
after update on user
for each row
begin
    update user set updated_at = current_timestamp where id = old.id;
end;