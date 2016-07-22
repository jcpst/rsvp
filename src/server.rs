use nickel::{HttpRouter, MediaType, Nickel, StaticFilesHandler};
use queries;
use rustc_serialize::json;

pub fn server() {
    let mut server = Nickel::new();

    server.utilize(StaticFilesHandler::new("public/"));

    // need 'resend invite' route

    // Have this return a pretty page with data on it.
    server.get("/confirm/:uuid",
               middleware! { |req, _res|
        let id = req.param("uuid").unwrap().parse().unwrap();
        queries::confirm(id);
        r#"Thank you for confirming! See you there."#
    });

    server.listen("localhost:6767");
}