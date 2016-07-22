use nickel::{HttpRouter, MediaType, Nickel, StaticFilesHandler};
use queries;
use rustc_serialize::json;

pub fn server() {
    let mut server = Nickel::new();

    server.utilize(StaticFilesHandler::new("public/"));

    // have a page that lets the user see if they were invited
    // by typing in thier email. If it exists, it will send them
    // the email with the confirmation link again.
    //
    // Calls a query to the database that matches on their email.
    // If the email matches, the invite gets emailed to their address
    // With the uuid needed for the confirmation click.

    // Have this return a pretty page with data on it.
    server.get("/confirm/:uuid",
               middleware! { |req, _res|
        let id = req.param("uuid").unwrap().parse().unwrap();
        queries::confirm(id);
        r#"Thank you for confirming! See you there."#
    });

    server.listen("localhost:6767");
}