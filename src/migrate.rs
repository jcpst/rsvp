extern crate csv;

use queries;

#[derive(RustcDecodable)]
struct Invitee {
    name: String,
    email: String,
}

pub fn migrate() {
    let mut reader = csv::Reader::from_file("invitees.csv").unwrap();
    for record in reader.decode() {
        let invitee: Invitee = record.unwrap();
        queries::create(invitee.name, invitee.email);
    }
}
