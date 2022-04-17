CREATE TABLE rooms (
    id serial primary key,
    name varchar(128) NOT NULL unique,
    ico varchar(256) NOT NULL default '',
    invite_link varchar(256) NOT NULL unique,
    owner int NOT NULL references users(id)
);

CREATE TABLE room_users(
    id serial primary key,
    room_id int references rooms on update cascade on delete cascade not null,
    user_id int references users on update cascade on delete cascade not null,
    is_admin boolean not null default false
);