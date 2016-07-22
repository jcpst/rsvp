DROP DATABASE IF EXISTS rsvp;
CREATE DATABASE rsvp;

\c rsvp;

CREATE TABLE invites (
	uuid UUID,
	name VARCHAR,
	email VARCHAR,
	confirmed BOOLEAN,
  invited BOOLEAN
);
