use actix_web::{web, HttpResponse, Responder, Scope};

use crate::request;
use crate::model;
use crate::store;

pub struct URL {
    store: store::url::URL,
}

impl Clone for URL {
    fn clone(&self) -> Self {
        URL {
            store: self.store.clone(),
        }
    }
}

impl URL {
    pub fn new(store: store::url::URL) -> Self {
        URL {
            store,
        }
    }

    async fn create(data: web::Data<Self>, url: web::Json<request::URL>) -> impl Responder {
        println!("get {:?}", url);

        let name: String;
        name = if url.name() == "-" {
            store::url::URL::random_key()
        } else {
          String::from(url.name())
        };

        let m = model::url::URL::new(url.url(), name.as_str());
        data.as_ref().store.store(&m).await;

        HttpResponse::Ok().json(m)
    }

    async fn fetch(data: web::Data<Self>, name: web::Path<String>) -> impl Responder {
        println!("get {}", name);

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
            .app_data(data).route("/url", web::post().to(Self::create)).route("/url/{name}", web::get().to(Self::fetch))
    }
}
