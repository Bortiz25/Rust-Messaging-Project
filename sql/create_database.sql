CREATE DATABASE rust_messages;

\connect rust_messages;

CREATE TABLE users (
  user_id SERIAL PRIMARY KEY NOT NULL,
  username VARCHAR(20) NOT NULL,
  password VARCHAR(255) NOT NULL
);

CREATE TABLE chats (
  chat_id SERIAL PRIMARY KEY NOT NULL,
  chat_history TEXT DEFAULT ''
);

CREATE TABLE user_to_chat (
  user_to_chat_id SERIAL PRIMARY KEY,
  user_id INT NOT NULL REFERENCES users(user_id),
  chat_id INT NOT NULL REFERENCES chats(chat_id)
);
