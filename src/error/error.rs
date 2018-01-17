use failure::{Fail};


#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "Configuration Error")]
    ConfigError,
}
