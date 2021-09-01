use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Database {
    address: String,
    name: String,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    host: String,
    port: u32,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    server: Server,
    database: Database,
}

impl Settings {
    pub fn new() -> Result<Self, config::ConfigError> {
        let mut settings = config::Config::new();

        settings
            .merge(config::File::with_name("config/default"))?
            .merge(config::Environment::with_prefix("FESGHEL"))?;

        settings.merge(config::File::with_name("settings")).ok();

        settings.try_into()
    }

    pub fn server(&self) -> &Server {
        &self.server
    }

    pub fn database(&self) -> &Database {
        &self.database
    }
}

impl Server {
    pub fn port(&self) -> u32 {
        self.port
    }

    pub fn host(&self) -> &str {
        self.host.as_str()
    }
}

impl Database {
    pub fn address(&self) -> &str {
        self.address.as_str()
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}
