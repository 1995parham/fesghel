// `mod` declarations make submodules available to this crate.
// Rust looks for `database.rs` or `database/mod.rs` for each declaration.
mod database;
mod handler;
mod model;
mod request;
mod setting;
mod store;

// `use` brings items into scope, avoiding repetitive full paths.
use actix_web::{App, HttpServer, web};

use setting::Settings;

// Procedural macro that sets up the async runtime (tokio).
// `async fn` declares an asynchronous function that returns a Future.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // `expect()` unwraps a Result, panicking with a message if it's Err.
    // Use for unrecoverable errors during initialization.
    simple_logger::init_with_level(log::Level::Info).expect("logger initiation failed");

    let setting = Settings::new().expect("loading configuration failed");

    // `.await` suspends execution until the Future completes.
    // This is non-blocking - other tasks can run while waiting.
    let db = database::connect(setting.database()).await;

    // `move` keyword transfers ownership of captured variables (`db`) into the closure.
    // Required here because the closure outlives the current function scope.
    HttpServer::new(move || {
        // `clone()` creates a deep copy. Required because each worker thread
        // needs its own instance. `Database` implements `Clone` trait.
        let store = store::Url::new(db.clone());

        // Builder pattern: chain method calls that return `Self` for fluent API.
        App::new()
            .service(crate::handler::url::register(
                crate::handler::url::State::new(store),
                web::scope("/api"),
            ))
            .service(crate::handler::healthz::register(web::scope("")))
    })
    .workers(12)
    // `format!` macro creates a formatted String, similar to printf.
    .bind(format!(
        "{}:{}",
        setting.server().host(),
        setting.server().port()
    ))?  // `?` operator: early return if Result is Err, otherwise unwrap Ok value.
    .run()
    .await
}
