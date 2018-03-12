use iron::prelude::*;
use iron::{status, Handler};
use router::Router;


pub struct JsonApi {}

struct ServerHandler {
    message: String,
}

impl Handler for ServerHandler {
    fn handle(&self, _req: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, self.message.clone())))
    }
}

impl JsonApi {
    pub fn start() {
        let handler = ServerHandler {
            message: "'xmz-server' JsonApi".to_string(),
        };
        let mut router = Router::new();
        router.get("/", handler, "index");

        println!("Starte json API: http://localhost:3000");
        Iron::new(router).http("localhost:3000").unwrap();
    }
}
