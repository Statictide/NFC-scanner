/*
create table audit_log (
    id integer primary key,
    user_id integer not null,
    action text not null,
    created_at timestamp not null default (datetime('now'))
);
*/

create table parent_history (
    parent_history_id integer primary key,
    user_id integer not null,
    entity_name text not null,
    old_parent_name text null,
    new_parent_name text null,
    created_at timestamp not null default (datetime('now'))
);