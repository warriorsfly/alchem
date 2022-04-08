CREATE TABLE users(
    id serial primary key,
    name varchar(255) NOT NULL unique,
    display_name varchar(255)NOT NULL default '',
    avatar varchar(255) NOT NULL default '',
    bio text NOT NULL default '',
    local boolean NOT NULL DEFAULT true
);