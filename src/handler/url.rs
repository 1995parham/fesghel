use actix_web::http::header;
use actix_web::{HttpResponse, Responder, Scope, get, post, web};

// `crate::` refers to the root of the current crate (project).
use crate::model;
use crate::request;
use crate::store;

// A struct holding application state. In Rust, structs are the primary way
// to create custom types that group related data together.
pub struct State {
    store: store::Url,
}

// `impl` block defines methods associated with a type.
// Methods can take `self`, `&self`, or `&mut self` as first parameter.
impl State {
    // Associated function (no self) - called via `State::new()`.
    // `Self` is an alias for the implementing type (`State`).
    pub fn new(store: store::Url) -> Self {
        // Field init shorthand: `store: store` can be written as just `store`.
        State { store }
    }
}

// Attribute macro: transforms the function into an HTTP POST handler.
// Actix uses procedural macros to generate routing code at compile time.
#[post("/urls")]
// `impl Responder` is a return-position impl trait - the function returns
// some type that implements Responder, without specifying which concrete type.
async fn create(data: web::Data<State>, url: web::Json<request::Url>) -> impl Responder {
    // `log::info!` is a macro. The `{url:?}` uses Debug formatting (`:?`).
    log::info!("get {url:?}");

    // `if let` destructures a pattern. Here it extracts Err variant.
    // More concise than full `match` when you only care about one variant.
    if let Err(err) = url.validate() {
        log::warn!("validation failed: {err}");
        return HttpResponse::BadRequest().json(err.to_string());
    }

    // `if-else` is an expression in Rust - it returns a value.
    // Both branches must return the same type.
    let name = if url.name() == "-" {
        store::Url::random_key()
    } else {
        // `String::from()` creates an owned String from a string slice (&str).
        String::from(url.name())
    };

    let m = model::Url::new(url.url(), name.as_str());
    // `match` is exhaustive pattern matching - all variants must be handled.
    // `Ok(..)` uses `..` to ignore the inner value we don't need.
    match data.store.store(&m).await {
        Ok(..) => HttpResponse::Ok().json(m.key()),
        Err(err) => {
            log::error!("{err}");
            HttpResponse::InternalServerError().json("Something went wrong")
        }
    }
}

#[get("/{name}")]
// `web::Path<String>` extracts path parameters. Actix deserializes `{name}` from URL.
async fn fetch(data: web::Data<State>, name: web::Path<String>) -> impl Responder {
    log::info!("get {name}");

    let url = data.store.fetch(name.as_str()).await;

    // Pattern matching on Option<T>: Some(value) or None.
    // Option is Rust's way of handling nullable values safely.
    match url {
        Some(url) => HttpResponse::TemporaryRedirect()
            // Tuple syntax `(a, b)` creates an anonymous pair.
            .insert_header((header::LOCATION, url.url()))
            .finish(),
        None => HttpResponse::NotFound().finish(),
    }
}

// `pub fn` makes this function public (accessible from other modules).
// Without `pub`, items are private to their module by default.
pub fn register(state: State, scope: Scope) -> Scope {
    // `web::Data` wraps state in Arc for thread-safe shared ownership.
    let data = web::Data::new(state);
    scope.app_data(data).service(create).service(fetch)
}
