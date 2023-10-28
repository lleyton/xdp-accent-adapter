use serde::Deserialize;
use tokio::fs::read_to_string;

#[derive(Deserialize, Default, Debug)]
pub struct Libadwaita {
    pub enabled: bool,
}

#[derive(Deserialize, Default, Debug)]
pub struct AdwGtk3 {
    pub enabled: bool,
}

#[derive(Deserialize, Default, Debug)]
#[serde(default)]
pub struct Backends {
    pub libadwaita: Libadwaita,
    pub adw_gtk3: AdwGtk3,
}

#[derive(Deserialize, Default, Debug)]
pub struct Config {
    pub backends: Backends,
}

impl Config {
    pub async fn load() -> color_eyre::Result<Option<Config>> {
        let base_directories = xdg::BaseDirectories::new()?;
        let path = base_directories.find_config_file("xdp-accent-adapter/config.toml");

        if let Some(path) = path {
            let data = read_to_string(path).await?;
            return Ok(toml::from_str(&data)?);
        }

        Ok(None)
    }
}
