CREATE TABLE entity (
    id INTEGER PRIMARY KEY,
    tag_id TEXT not null,
    name VARCHAR(255),
    owner TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP not null,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP not null
);

CREATE TRIGGER update_entity_updated_at
AFTER UPDATE ON entity
FOR EACH ROW
BEGIN
    UPDATE entity SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.id;
END;


create table user (
    id INTEGER PRIMARY KEY,
    name TEXT not null,
    username TEXT not null,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP not null,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP not null
);

CREATE TRIGGER update_user_updated_at
AFTER UPDATE ON user
FOR EACH ROW
BEGIN
    UPDATE user SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.id;
END;