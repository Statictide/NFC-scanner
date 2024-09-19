alter table entity {
    add column last_changed timestamp not null default current_timestamp
};