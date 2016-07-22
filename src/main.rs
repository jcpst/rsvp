#[macro_use]
extern crate clap;
extern crate dotenv;
#[macro_use]
extern crate nickel;
extern crate rustc_serialize;

mod queries;
mod mailer;
mod migrate;
mod server;

use clap::App;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    match matches.subcommand_name() {
        Some("server") => server::server(),
        Some("migrate") => migrate::migrate(),
        Some("mailer") => mailer::mailer(),
        _ => (), 
    }
}
