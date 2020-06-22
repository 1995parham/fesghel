use actix_web::{web, HttpResponse, Responder, Scope};
use log;

use crate::request;
use crate::model;
use crate::store;

pub struct URL {
    store: store::URL,
}

impl Clone for URL {
    fn clone(&self) -> Self {
        URL {
            store: self.store.clone(),
        }
    }
}

impl URL {
    pub fn new(store: store::URL) -> Self {
        URL {
            store,
        }
    }

    async fn create(data: web::Data<Self>, url: web::Json<request::URL>) -> impl Responder {
        log::info!("get {:?}", url);

        let name: String;
        name = if url.name() == "-" {
            store::URL::random_key()
        } else {
          String::from(url.name())
        };

        let m = model::URL::new(url.url(), name.as_str());
        match data.as_ref().store.store(&m).await {
            Ok(..) => HttpResponse::Ok().json(m.key()),
            Err(err) => {
                log::error!("{}", err);
                HttpResponse::InternalServerError().json("Something went wrong")
            },
    }
    }

    async fn fetch(data: web::Data<Self>, name: web::Path<String>) -> impl Responder {
        log::info!("get {}", name);

        let url = data.as_ref().store.fetch(name.as_str()).await;

        match url {
            Some(url) =>
                HttpResponse::Found().set_header("Location", url.url()).finish(),
            None =>
                HttpResponse::NotFound().finish(),
        }


    }

    pub fn register(self, scope: Scope) -> Scope {
        let data = web::Data::new(self);
        scope
            .app_data(data).route("/urls", web::post().to(Self::create)).route("/{name}", web::get().to(Self::fetch))
    }
}
