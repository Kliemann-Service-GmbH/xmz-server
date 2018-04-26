use std::path::PathBuf;

#[derive(Debug)]
#[derive(Deserialize, Configure)]
#[serde(default)]
pub struct Config {
    pub configuration_path: PathBuf,
    pub runtime_info_path: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            configuration_path: PathBuf::from("/boot/xmz-server.toml"),
            runtime_info_path: PathBuf::from("/var/cache/xmz-server/status"),
        }
    }
}
