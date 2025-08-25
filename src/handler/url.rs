use actix_web::http::header;
use actix_web::{HttpResponse, Responder, Scope, get, post, web};

use crate::model;
use crate::request;
use crate::store;

pub struct State {
    store: store::Url,
}

impl State {
    pub fn new(store: store::Url) -> Self {
        State { store }
    }
}

#[post("/urls")]
async fn create(data: web::Data<State>, url: web::Json<request::Url>) -> impl Responder {
    log::info!("get {url:?}");

    let name = if url.name() == "-" {
        store::Url::random_key()
    } else {
        String::from(url.name())
    };

    let m = model::Url::new(url.url(), name.as_str());
    match data.store.store(&m).await {
        Ok(..) => HttpResponse::Ok().json(m.key()),
        Err(err) => {
            log::error!("{err}");
            HttpResponse::InternalServerError().json("Something went wrong")
        }
    }
}

#[get("/{name}")]
async fn fetch(data: web::Data<State>, name: web::Path<String>) -> impl Responder {
    log::info!("get {name}");

    let url = data.store.fetch(name.as_str()).await;

    match url {
        Some(url) => HttpResponse::TemporaryRedirect()
            .insert_header((header::LOCATION, url.url()))
            .finish(),
        None => HttpResponse::NotFound().finish(),
    }
}

pub fn register(state: State, scope: Scope) -> Scope {
    let data = web::Data::new(state);
    scope.app_data(data).service(create).service(fetch)
}
