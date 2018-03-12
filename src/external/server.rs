/// Externe Darstellung des Servers
use serde::ser::{Serialize};
use serde::de::{Deserialize};
use server::Server as InternalServer;


#[derive(Default)]
#[derive(Serialize, Deserialize)]
#[serde(default, rename_all="PascalCase")]
pub struct Server {
    pub service_interval: u32,
}


impl From<InternalServer> for Server {
    fn from(internal: InternalServer) -> Self {
        Server {
            service_interval: internal.service_interval,
        }
    }
}
