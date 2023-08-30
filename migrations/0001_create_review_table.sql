CREATE TABLE reviews(
    id bigserial primary key,
    author_id varchar(255) NOT NUll,
    camp_id bigint NOT NULL,
    ctime timestamp with time zone DEFAULT now(),
    title varchar(255) NOT NULL,
    body varchar(255) DEFAULT '',
    rating int NOT NULL
);