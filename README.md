Simple RSVP
===========

Heavily inspired by [this][1] blog post with a simple RSVP tool built with node.

As a node developer starting to learn Rust, this looked like a fun experiment.

Usage
-----

Requires [Rust][2] and [Postgres][3].

### Build Project

- `git clone` this project.
- `cd rsvp`
- `cargo build`

### Initialize Database

```
psql -f db/init.sql
```

### Add config files (at root of project)

#### `.env`

Required variables:

```
EMAIL_HOST=smtp.example.com
EMAIL_PORT=587
EMAIL_ADDRESS=joe@example.com
EMAIL_PASSWORD=123secrect
POSTGRES_CONNECTION_STRING=postgres://postgres@localhost:5432/rsvp
```

#### `invitees.csv`

This will be the list of people that get emailed invitations:

```
name,email
Joe Bob,jb@example.com
Tom Bomb,tb@example.com
```

### Commands

- `cargo run -- migrate` 
  - will read the info from your invitees.csv file and add them to the database
    as pending invitations
- `cargo run -- mailer`
  - will send invitations to the people that haven't been invited yet, and update
    their record as 'invited'
- `cargo run -- server`
  - runs a [Nickel][4] server that accepts invitations. 

TODO
----

- Create a template for the email body
- Serve a page that lets you enter your email, and if it's in the database,
  will resend the invitation.

[1]: http://ditrospecta.com/node/2016/07/11/simple-rsvp-node.html
[2]: https://www.rust-lang.org
[3]: https://www.postgresql.org/
[4]: http://nickel.rs/