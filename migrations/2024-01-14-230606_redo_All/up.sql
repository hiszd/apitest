-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

SELECT diesel_manage_updated_at('users');

CREATE TYPE statustype AS ENUM ('open', 'closed', 'working', 'assigned', 'unassigned');
CREATE TYPE tickettype AS ENUM ('hardware', 'software', 'employee', 'email');

CREATE TABLE tickets (
  id SERIAL PRIMARY KEY,
  count INT NOT NULL,
  subject TEXT NOT NULL,
  description TEXT NOT NULL,
  status statustype NOT NULL DEFAULT 'open',
  ticktype tickettype NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

SELECT diesel_manage_updated_at('tickets');

CREATE TABLE tickets_authors (
  ticket_id INTEGER REFERENCES tickets(id),
  author_id INTEGER REFERENCES users(id),
  PRIMARY KEY(ticket_id, author_id)
);
