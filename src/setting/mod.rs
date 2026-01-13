use serde::Deserialize;

// Nested structs model hierarchical configuration.
// Serde maps TOML sections like `[database]` to struct fields.
#[derive(Debug, Deserialize)]
pub struct Database {
    address: String,
    name: String,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    host: String,
    // `u32` is unsigned 32-bit integer. Rust has explicit integer sizes:
    // i8/u8, i16/u16, i32/u32, i64/u64, i128/u128, isize/usize.
    port: u32,
}

// Composition: Settings contains other structs as fields.
// This creates a tree structure matching the config file layout.
#[derive(Debug, Deserialize)]
pub struct Settings {
    server: Server,
    database: Database,
}

impl Settings {
    // Returns `Result<Self, Error>` - fallible constructor pattern.
    // Caller must handle potential configuration errors.
    pub fn new() -> Result<Self, config::ConfigError> {
        // Builder pattern: construct complex objects step by step.
        // Each method returns the builder for chaining.
        let settings = config::Config::builder();

        settings
            // Configuration sources are layered - later sources override earlier.
            // 1. Base defaults from file
            .add_source(config::File::with_name("config/default"))
            // 2. Environment variables (FESGHEL_SERVER__PORT, etc.)
            // Double underscore `__` represents nesting in env vars.
            .add_source(config::Environment::with_prefix("FESGHEL"))
            // 3. Optional local overrides (`.required(false)` won't fail if missing)
            .add_source(config::File::with_name("settings").required(false))
            // `build()` finalizes and returns Result<Config, Error>
            .build()?
            // `try_deserialize()` converts Config to our Settings struct via Serde.
            // Returns Result<Settings, Error>.
            .try_deserialize()
    }

    // Getter returns reference to avoid copying the nested struct.
    // `&Server` is a borrow - caller can read but not own the data.
    pub fn server(&self) -> &Server {
        &self.server
    }

    pub fn database(&self) -> &Database {
        &self.database
    }
}

// Each struct gets its own impl block for its methods.
// This separation keeps related code together.
impl Server {
    // Primitive types like u32 are Copy - returning by value is cheap.
    // No need to return reference for small, copyable types.
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
