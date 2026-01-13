// Prometheus metrics module for application observability.
// Custom metrics complement the HTTP metrics from actix-web-prom.

use lazy_static::lazy_static;
use prometheus::{Counter, Gauge, Histogram, HistogramOpts, IntCounterVec, Opts, Registry};

// `lazy_static!` creates static variables that are initialized on first access.
// Useful for metrics that need runtime initialization.
lazy_static! {
    // Registry holds all metrics. We create a custom one to share with actix-web-prom.
    pub static ref REGISTRY: Registry = Registry::new();

    // Gauge: a metric that can go up or down (current value).
    // Used for things like worker count, active connections, queue size.
    pub static ref WORKERS: Gauge = Gauge::with_opts(
        Opts::new("fesghel_workers", "Number of HTTP server worker threads")
    ).expect("metric can be created");

    // Counter: a metric that only increases (cumulative total).
    // Used for counting events like requests, errors, items processed.
    pub static ref URLS_CREATED: Counter = Counter::with_opts(
        Opts::new("fesghel_urls_created_total", "Total number of shortened URLs created")
    ).expect("metric can be created");

    // IntCounterVec: a counter with labels for dimensional data.
    // Labels allow slicing metrics by different dimensions.
    pub static ref ERRORS: IntCounterVec = IntCounterVec::new(
        Opts::new("fesghel_errors_total", "Total number of errors by type"),
        &["type"]  // Label name - values: "duplicate_key", "database", "validation"
    ).expect("metric can be created");

    // Info metric for build/version metadata (implemented as gauge with labels).
    pub static ref APP_INFO: Gauge = Gauge::with_opts(
        Opts::new("fesghel_app_info", "Application build information")
            .const_label("version", env!("CARGO_PKG_VERSION"))
    ).expect("metric can be created");

    // Database operation counters - track total reads and writes.
    pub static ref DB_READS: Counter = Counter::with_opts(
        Opts::new("fesghel_db_reads_total", "Total number of database read operations")
    ).expect("metric can be created");

    pub static ref DB_WRITES: Counter = Counter::with_opts(
        Opts::new("fesghel_db_writes_total", "Total number of database write operations")
    ).expect("metric can be created");

    // Histogram: tracks distribution of values (latency, size, etc.).
    // Buckets define the boundaries for grouping observations.
    // Default buckets: 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1, 2.5, 5, 10
    pub static ref DB_READ_DURATION: Histogram = Histogram::with_opts(
        HistogramOpts::new(
            "fesghel_db_read_duration_seconds",
            "Database read operation duration in seconds"
        )
        // Custom buckets optimized for database operations (in seconds).
        .buckets(vec![0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0])
    ).expect("metric can be created");

    pub static ref DB_WRITE_DURATION: Histogram = Histogram::with_opts(
        HistogramOpts::new(
            "fesghel_db_write_duration_seconds",
            "Database write operation duration in seconds"
        )
        .buckets(vec![0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0])
    ).expect("metric can be created");
}

/// Register all custom metrics with the registry.
/// Called once at application startup before creating the prometheus middleware.
pub fn register_metrics() {
    REGISTRY
        .register(Box::new(WORKERS.clone()))
        .expect("WORKERS metric registered");

    REGISTRY
        .register(Box::new(URLS_CREATED.clone()))
        .expect("URLS_CREATED metric registered");

    REGISTRY
        .register(Box::new(ERRORS.clone()))
        .expect("ERRORS metric registered");

    REGISTRY
        .register(Box::new(APP_INFO.clone()))
        .expect("APP_INFO metric registered");

    REGISTRY
        .register(Box::new(DB_READS.clone()))
        .expect("DB_READS metric registered");

    REGISTRY
        .register(Box::new(DB_WRITES.clone()))
        .expect("DB_WRITES metric registered");

    REGISTRY
        .register(Box::new(DB_READ_DURATION.clone()))
        .expect("DB_READ_DURATION metric registered");

    REGISTRY
        .register(Box::new(DB_WRITE_DURATION.clone()))
        .expect("DB_WRITE_DURATION metric registered");

    // Set app info to 1 (presence indicator with version label).
    APP_INFO.set(1.0);
}

/// Set the worker count gauge.
/// Called at startup after worker count is determined.
pub fn set_workers(count: usize) {
    WORKERS.set(count as f64);
}

/// Increment the URLs created counter.
/// Called after successfully storing a new shortened URL.
pub fn inc_urls_created() {
    URLS_CREATED.inc();
}

/// Increment error counter by type.
/// Error types: "duplicate_key", "database", "validation"
pub fn inc_error(error_type: &str) {
    ERRORS.with_label_values(&[error_type]).inc();
}

/// Record a database read operation with its duration.
/// Duration should be in seconds (use `Instant::elapsed().as_secs_f64()`).
pub fn observe_db_read(duration_secs: f64) {
    DB_READS.inc();
    DB_READ_DURATION.observe(duration_secs);
}

/// Record a database write operation with its duration.
/// Duration should be in seconds (use `Instant::elapsed().as_secs_f64()`).
pub fn observe_db_write(duration_secs: f64) {
    DB_WRITES.inc();
    DB_WRITE_DURATION.observe(duration_secs);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn workers_gauge_can_be_set() {
        set_workers(12);
        assert_eq!(WORKERS.get() as usize, 12);
    }

    #[test]
    fn urls_counter_increments() {
        let before = URLS_CREATED.get() as u64;
        inc_urls_created();
        assert_eq!(URLS_CREATED.get() as u64, before + 1);
    }

    #[test]
    fn error_counter_increments_by_type() {
        let before = ERRORS.with_label_values(&["test_error"]).get();
        inc_error("test_error");
        assert_eq!(ERRORS.with_label_values(&["test_error"]).get(), before + 1);
    }

    #[test]
    fn db_read_increments_counter_and_histogram() {
        let before = DB_READS.get() as u64;
        let count_before = DB_READ_DURATION.get_sample_count();
        observe_db_read(0.015); // 15ms
        assert_eq!(DB_READS.get() as u64, before + 1);
        assert_eq!(DB_READ_DURATION.get_sample_count(), count_before + 1);
    }

    #[test]
    fn db_write_increments_counter_and_histogram() {
        let before = DB_WRITES.get() as u64;
        let count_before = DB_WRITE_DURATION.get_sample_count();
        observe_db_write(0.025); // 25ms
        assert_eq!(DB_WRITES.get() as u64, before + 1);
        assert_eq!(DB_WRITE_DURATION.get_sample_count(), count_before + 1);
    }
}
