CREATE TABLE posts (
  id INTEGER NOT NULL PRIMARY KEY,
  title VARCHAR NOT NULL,
  body TEXT NOT NULL,
  published BOOLEAN NOT NULL DEFAULT 'f'
);


CREATE TABLE users (
  user_id INTEGER NOT NULL PRIMARY KEY,
  screen_name VARCHAR NOT NULL,
  first_name VARCHAR,
  last_name VARCHAR,
  anrede VARCHAR,
  geburtstag INTEGER,
  email VARCHAR,
  portrait INTEGER,
  original_site_id INTEGER
);

CREATE TABLE files (
  file_hotlink_url VARCHAR NOT NULL,
  file_title VARCHAR NOT NULL,
  file_id INTEGER NOT NULL PRIMARY KEY,
  file_type_id INTEGER NOT NULL,
  site_id INTEGER NOT NULL,
  folder_id INTEGER NOT NULL
);

CREATE TABLE sites (
  site_id  INTEGER NOT NULL PRIMARY KEY,
  parent_site_id INTEGER NOT NULL,
friendly_url VARCHAR NOT NULL,
theme_id INTEGER NOT NULL,
additional_javascript VARCHAR NOT NULL,
additional_css VARCHAR NOT NULL,
title VARCHAR NOT NULL

);

CREATE TABLE pages (
  site_id INTEGER NOT NULL,
  page_id INTEGER NOT NULL PRIMARY KEY,
title VARCHAR NOT NULL,
friendly_url VARCHAR NOT NULL,
parent_page_id INTEGER NOT NULL,
additional_javascript VARCHAR NOT NULL,
additional_css VARCHAR NOT NULL

);

CREATE TABLE portlets (
  portlet_id  INTEGER NOT NULL PRIMARY KEY,
  page_id INTEGER NOT NULL,
  page_internal_path VARCHAR NOT NULL,
  portlet_type VARCHAR NOT NULL,
  title VARCHAR NOT NULL,
  config VARCHAR NOT NULL
);


CREATE TABLE structures (
  title VARCHAR NOT NULL,
structure_id  INTEGER NOT NULL PRIMARY KEY,
publish_date INTEGER NOT NULL,
expiration_date INTEGER NOT NULL,
jsonStructure VARCHAR NOT NULL,
parent_structure_id INTEGER NOT NULL,
is_trash BOOLEAN NOT NULL DEFAULT 'f',
is_draft BOOLEAN NOT NULL DEFAULT 'f',
create_date INTEGER NOT NULL,
modified_date INTEGER NOT NULL,
create_user_id INTEGER NOT NULL,
modified_user_id INTEGER NOT NULL,
site_id INTEGER NOT NULL

);


CREATE TABLE templates (
  template_id INTEGER NOT NULL PRIMARY KEY
);


CREATE TABLE articles (
  title VARCHAR NOT NULL,
post_id INTEGER NOT NULL PRIMARY KEY,
friendly_url VARCHAR NOT NULL,
publish_date INTEGER NOT NULL,
expiration_date INTEGER NOT NULL,
structure_id INTEGER NOT NULL,
content VARCHAR NOT NULL,
folder_id INTEGER NOT NULL,
is_trash BOOLEAN NOT NULL DEFAULT 'f',
is_draft BOOLEAN NOT NULL DEFAULT 'f',
create_date INTEGER NOT NULL,
modified_date INTEGER NOT NULL,
create_user_id INTEGER NOT NULL,
modified_user_id INTEGER NOT NULL,
site_id INTEGER NOT NULL

);