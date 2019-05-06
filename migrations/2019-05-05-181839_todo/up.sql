CREATE TABLE todo(
    todo_id serial PRIMARY KEY,
    task VARCHAR (64) UNIQUE NOT NULL,
    priority smallint NOT NULL
);