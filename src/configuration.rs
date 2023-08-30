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
    pub data_entries_path: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub path: String,
}

#[derive(serde::Deserialize)]
pub struct SearchEngineSettings {
    pub path: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::File::with_name("config/config"))
        .build()?;
    settings.try_deserialize()
}
