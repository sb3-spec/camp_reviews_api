CREATE TABLE IF NOT EXISTS camp_requests(
    id bigserial primary key,
    name varchar(255) DEFAULT '' NOT NULL,
    description varchar(255) DEFAULT '' NOT NULL,
    tags varchar(255)[] DEFAULT array[]::varchar(255)[],
    image_urls text[] DEFAULT array[]::text[],
    phone_number varchar(255) DEFAULT '' NOT NULL,
    email varchar(255) DEFAULT '' NOT NULL,
    street_address varchar(255) DEFAULT '' NOT NULL,
    city varchar(255) DEFAULT '' NOT NULL,
    state varchar(255) DEFAULT '' NOT NULL,
    zip_code varchar(255) DEFAULT '' NOT NULL,
    country varchar(255) DEFAULT '' NOT NULL,
    apt_suite_other varchar(255) DEFAULT '',
    website varchar(255) DEFAULT '',
    user_id varchar(255) DEFAULT '' NOT NULL,

    CONSTRAINT fk_users FOREIGN KEY (user_id) REFERENCES users(supabase_id)
);