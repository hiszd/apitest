-- Your SQL goes here
CREATE TYPE statustype AS ENUM ('open', 'closed', 'working', 'assigned', 'unassigned');
CREATE TYPE tickettype AS ENUM ('hardware', 'software', 'employee', 'email');
CREATE TYPE role AS ENUM ('user', 'admin', 'agent', 'manager');

ALTER TABLE tickets ADD COLUMN status statustype;
ALTER TABLE tickets ADD COLUMN ticktype tickettype;
