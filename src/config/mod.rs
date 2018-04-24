use configure::Configure;
use std::path::PathBuf;

#[derive(Debug)]
#[derive(Deserialize, Configure)]
#[serde(default)]
pub struct Config {
    configuration_path: PathBuf,
    runtime_info_path: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            configuration_path: PathBuf::from("/boot/xmz-server.toml"),
            runtime_info_path: PathBuf::from("/var/cache/xmz-server/status"),
        }
    }
}
