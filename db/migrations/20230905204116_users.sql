create type roles_enum as ENUM ('STUDENT', 'ADMIN', 'TEACHER');
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

create table if NOT exists exams (
    id BIGINT PRIMARY KEY,
    name text NOT NULL,
    outof INT not null,
    grades BIGINT[] not null
);

create table if NOT exists grades (
    id BIGINT PRIMARY KEY,
    exam_id BIGINT not null,
    user_id BIGINT NOT NULL,
    grade INT not null,
    paper text not null,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (exam_id) REFERENCES exams(id) ON DELETE CASCADE
);


