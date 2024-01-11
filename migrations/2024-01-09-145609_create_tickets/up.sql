-- Your SQL goes here
CREATE TYPE statustype AS ENUM ('open', 'closed', 'working', 'assigned', 'unassigned');
CREATE TYPE tickettype AS ENUM ('hardware', 'software', 'employee', 'email');

CREATE TABLE tickets (
  id SERIAL PRIMARY KEY,
  index INT NOT NULL,
  subject TEXT NOT NULL,
  description TEXT NOT NULL,
  status statustype NOT NULL DEFAULT 'open',
  ticktype tickettype NOT NULL,
  timespent FLOAT DEFAULT 0
);
