CREATE DATABASE rust_messages;

\connect rust_messages;

CREATE TABLE users (
  user_id SERIAL PRIMARY KEY NOT NULL,
  username VARCHAR(20) NOT NULL,
  password VARCHAR(255) NOT NULL
);

CREATE TABLE chats (
  chat_id SERIAL PRIMARY KEY NOT NULL
);

CREATE TABLE messages (
  message_id SERIAL PRIMARY KEY NOT NULL,
  chat_id INT NOT NULL REFERENCES chats(chat_id),
  sent_from INT NOT NULL REFERENCES users(user_id),
  message TEXT NOT NULL
);

CREATE TABLE user_to_chat (
  user_to_chat_id SERIAL PRIMARY KEY,
  user_id INT NOT NULL REFERENCES users(user_id),
  chat_id INT NOT NULL REFERENCES chats(chat_id)
);
