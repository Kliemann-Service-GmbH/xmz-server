use iron::{status, Handler};
use iron::prelude::*;
use prelude::*;
use router::Router;
use server::Server;
use external::server::Server as ExternalServer;
use serde_json;

#[derive(Clone)]
pub struct JsonApi {
    api_url: String,
}

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
    pub fn new(api_url: String) -> Self {
        JsonApi {
            api_url,
        }
    }

    pub fn start(&self, server: ExternalServer) {
        let handler = ServerHandler::new(server);
        let mut router = Router::new();
        router.get("/", handler, "index");

        println!("Starte json API: http://{}", &self.api_url);
        Iron::new(router).http(&self.api_url).unwrap();
    }
}
