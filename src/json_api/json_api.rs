use iron::{status, Handler};
use iron::prelude::*;
use prelude::*;
use router::Router;
use server::Server;
use external::server::Server as ExternalServer;
use serde_json;


pub struct JsonApi {}

struct ServerHandler {
    server: ExternalServer,
}
impl ServerHandler {
    fn new(server: ExternalServer) -> Self {
        ServerHandler {
            server,
        }
    }
}

impl Handler for ServerHandler {
    fn handle(&self, _req: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, serde_json::to_string(&self.server).unwrap())))
    }
}

impl JsonApi {
    pub fn start(server: ExternalServer) {
        let handler = ServerHandler::new(server);
        let mut router = Router::new();
        router.get("/", handler, "index");

        println!("Starte json API: http://localhost:3000");
        Iron::new(router).http("localhost:3000").unwrap();
    }
}
