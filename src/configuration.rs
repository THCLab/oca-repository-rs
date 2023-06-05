#[derive(serde::Deserialize)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub database: DatabaseSettings,
    pub search_engine: SearchEngineSettings,
}

#[derive(serde::Deserialize)]
pub struct ApplicationSettings {
    pub host: String,
    pub port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub path: String,
}

#[derive(serde::Deserialize)]
pub struct SearchEngineSettings {
    pub url: String,
    pub api_key: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::File::from(std::path::Path::new("/tmp/config.yml")))
        .build()?;
    settings.try_deserialize()
}
