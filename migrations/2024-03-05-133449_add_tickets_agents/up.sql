-- Your SQL goes here

CREATE TABLE tickets_agents (
  ticket_id INTEGER REFERENCES tickets(id),
  agent_id INTEGER REFERENCES users(id),
  PRIMARY KEY(ticket_id, agent_id)
);

