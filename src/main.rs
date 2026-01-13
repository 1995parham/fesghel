// `mod` declarations make submodules available to this crate.
// Rust looks for `database.rs` or `database/mod.rs` for each declaration.
mod database;
mod handler;
mod metrics;
mod model;
mod request;
mod setting;
mod store;

// `use` brings items into scope, avoiding repetitive full paths.
use actix_web::{App, HttpServer, web};
use actix_web_prom::PrometheusMetricsBuilder;

use setting::Settings;

// Worker count as constant - can be made configurable via settings.
const WORKERS: usize = 12;

// Procedural macro that sets up the async runtime (tokio).
// `async fn` declares an asynchronous function that returns a Future.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // `expect()` unwraps a Result, panicking with a message if it's Err.
    // Use for unrecoverable errors during initialization.
    simple_logger::init_with_level(log::Level::Info).expect("logger initiation failed");

    let setting = Settings::new().expect("loading configuration failed");

    // Register custom metrics before building the prometheus middleware.
    metrics::register_metrics();
    metrics::set_workers(WORKERS);

    // Build prometheus middleware with custom registry.
    // This exposes /metrics endpoint and tracks HTTP request metrics.
    let prometheus = PrometheusMetricsBuilder::new("fesghel")
        .endpoint("/metrics")
        .registry(metrics::REGISTRY.clone())
        .build()
        .expect("prometheus metrics builder failed");

    // `.await` suspends execution until the Future completes.
    // This is non-blocking - other tasks can run while waiting.
    let db = database::connect(setting.database()).await;

    // Create store before server starts - `new()` is async to create indexes.
    // Store is created once and cloned for each worker thread.
    let store = store::Url::new(db).await;

    log::info!(
        "starting server on {}:{} with {} workers",
        setting.server().host(),
        setting.server().port(),
        WORKERS
    );

    // `move` keyword transfers ownership of captured variables (`store`) into the closure.
    // Required here because the closure outlives the current function scope.
    HttpServer::new(move || {
        // `clone()` creates a deep copy. Required because each worker thread
        // needs its own instance. `Url` implements `Clone` trait.
        let store = store.clone();

        // Builder pattern: chain method calls that return `Self` for fluent API.
        App::new()
            // `.wrap()` adds middleware. Prometheus middleware tracks all requests.
            .wrap(prometheus.clone())
            .service(crate::handler::url::register(
                crate::handler::url::State::new(store),
                web::scope("/api"),
            ))
            .service(crate::handler::healthz::register(web::scope("")))
    })
    .workers(WORKERS)
    // `format!` macro creates a formatted String, similar to printf.
    .bind(format!(
        "{}:{}",
        setting.server().host(),
        setting.server().port()
    ))? // `?` operator: early return if Result is Err, otherwise unwrap Ok value.
    .run()
    .await
}
