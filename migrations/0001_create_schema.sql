CREATE TABLE IF NOT EXISTS users(
    supabase_id varchar(255) primary key,
    first_name varchar(255) DEFAULT '' NOT NULL,
    last_name varchar(255) DEFAULT '' NOT NULL,
    email varchar(255) DEFAULT '' NOT NULL,
    username varchar(255) DEFAULT '' NOT NULL
);
CREATE TABLE IF NOT EXISTS camps(
    id bigserial primary key,
    name varchar(255) DEFAULT '' NOT NULL,
    description varchar(255) DEFAULT '' NOT NULL,
    tags varchar(255)[] DEFAULT array[]::varchar(255)[],
    image_urls text[] DEFAULT array[]::text[],
    rating real,
    phone_number varchar(255) DEFAULT '' NOT NULL,
    email varchar(255) DEFAULT '' NOT NULL,
    street_address varchar(255) DEFAULT '' NOT NULL,
    city varchar(255) DEFAULT '' NOT NULL,
    state varchar(255) DEFAULT '' NOT NULL,
    zip_code varchar(255) DEFAULT '' NOT NULL,
    country varchar(255) DEFAULT '' NOT NULL,
    apt_suite_other varchar(255) DEFAULT '',
    website varchar(255) DEFAULT ''
);
CREATE TABLE IF NOT EXISTS reviews(
    id bigserial primary key,
    author_id varchar(255) NOT NUll,
    camp_id bigint NOT NULL,
    ctime timestamp with time zone DEFAULT now() NOT NULL,
    title varchar(255) NOT NULL,
    body varchar(255) DEFAULT '' NOT NULL,
    rating int NOT NULL,
    CONSTRAINT fk_camps FOREIGN KEY (camp_id) REFERENCES camps(id),
    CONSTRAINT fk_users FOREIGN KEY (author_id) REFERENCES users(supabase_id)
);
CREATE TABLE users_camps(
    camp_id bigint REFERENCES camps(id),
    user_id varchar(255) REFERENCES users(supabase_id),
    CONSTRAINT users_camps_pkey PRIMARY KEY (camp_id, user_id)
);