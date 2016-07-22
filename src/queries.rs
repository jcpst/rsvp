extern crate postgres;
extern crate uuid;

use dotenv::dotenv;
use self::postgres::{Connection, SslMode};
use std::env;

#[derive(RustcDecodable, RustcEncodable)]
pub struct Invite {
    uuid: Option<uuid::Uuid>,
    pub name: String,
    pub email: String,
    invited: bool,
    confirmed: bool,
}

impl Invite {
    fn new(uuid: Option<uuid::Uuid>, name: String, email: String) -> Invite {
        Invite {
            uuid: uuid,
            name: name,
            email: email,
            invited: false,
            confirmed: false,
        }
    }
}

fn env_var(var: &str) -> String {
    match env::var(var) {
        Ok(x) => x,
        _ => "".to_string(),
    }
}

fn connection() -> Connection {
    dotenv().ok();
    let conn_string: &str = &env_var("POSTGRES_CONNECTION_STRING");
    Connection::connect(conn_string, SslMode::None).unwrap()
}

pub fn pending() -> Vec<Invite> {
    let conn = connection();
    let mut invites: Vec<Invite> = Vec::new();
    let query: &str = "SELECT uuid, name, email, invited, confirmed FROM invites WHERE invited = false";

    for row in &conn.query(query, &[]).unwrap() {
        let invite = Invite::new(Some(row.get(0)), row.get(1), row.get(2));
        invites.push(invite);
    }

    invites
}

pub fn invited(email: String) {
    let conn = connection();
    let query = "UPDATE invites SET invited = true WHERE email = $1";
    conn.execute(query, &[&email]).unwrap();
}

pub fn create(name: String, email: String) {
    let conn = connection();
    let query: &str = "INSERT INTO invites (uuid, name, email, invited, confirmed)
                       VALUES ($1, $2, $3, false, false)";
    let id = uuid::Uuid::new_v4();
    conn.execute(query, &[&id, &name, &email]).unwrap();
}

pub fn confirm(id: uuid::Uuid) {
    let conn = connection();
    let query: &str = "UPDATE invites SET confirmed = true WHERE uuid = $1";
    conn.execute(query, &[&id]).unwrap();
}
