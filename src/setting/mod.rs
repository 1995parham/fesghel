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
        let settings = config::Config::builder();

        settings
            .add_source(config::File::with_name("config/default"))
            .add_source(config::Environment::with_prefix("FESGHEL"))
            .add_source(config::File::with_name("settings").required(false))
            .build()?
            .try_deserialize()
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
