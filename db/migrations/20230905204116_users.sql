create type roles_enum as ENUM (
    'STUDENT', 'ADMIN', 'TEACHER'
);
-- Add migration script here
create table if not exists users (
    id BIGINT PRIMARY KEY,
	display_name text not null,
	username text unique not null,
	password_hash text not null,
	role roles_enum not null
);

CREATE TABLE IF NOT EXISTS sessions (
    id BIGINT PRIMARY KEY,
    token VARCHAR(64) NOT NULL,
    user_id BIGINT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);