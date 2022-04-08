CREATE TABLE local_users (
    id serial primary key,
    user_id int references users  on update cascade on delete cascade not null unique,
    password_encrypted varchar(128) NOT NULL,
    salt varchar(128) NOT NULL,
    phone varchar(13)
);