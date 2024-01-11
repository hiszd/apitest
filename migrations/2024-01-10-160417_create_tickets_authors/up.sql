-- Your SQL goes here
CREATE TABLE tickets_authors (
  ticket_id INTEGER REFERENCES tickets(id),
  author_id INTEGER REFERENCES users(id),
  PRIMARY KEY(ticket_id, author_id)
);
