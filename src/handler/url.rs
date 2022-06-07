use actix_web::http::header;
use actix_web::{web, HttpResponse, Responder, Scope};

use crate::model;
use crate::request;
use crate::store;

pub struct Url {
    store: store::Url,
}

impl Clone for Url {
    fn clone(&self) -> Self {
        Url {
            store: self.store.clone(),
        }
    }
}

impl Url {
    pub fn new(store: store::Url) -> Self {
        Url { store }
    }

    async fn create(data: web::Data<Self>, url: web::Json<request::Url>) -> impl Responder {
        log::info!("get {:?}", url);

        let name = if url.name() == "-" {
            store::Url::random_key()
        } else {
            String::from(url.name())
        };

        let m = model::Url::new(url.url(), name.as_str());
        match data.as_ref().store.store(&m).await {
            Ok(..) => HttpResponse::Ok().json(m.key()),
            Err(err) => {
                log::error!("{}", err);
                HttpResponse::InternalServerError().json("Something went wrong")
            }
        }
    }

    async fn fetch(data: web::Data<Self>, name: web::Path<String>) -> impl Responder {
        log::info!("get {}", name);

        let url = data.as_ref().store.fetch(name.as_str()).await;

        match url {
            Some(url) => HttpResponse::TemporaryRedirect()
                .insert_header((header::LOCATION, url.url()))
                .finish(),
            None => HttpResponse::NotFound().finish(),
        }
    }

    pub fn register(self, scope: Scope) -> Scope {
        let data = web::Data::new(self);
        scope
            .app_data(data)
            .route("/urls", web::post().to(Self::create))
            .route("/{name}", web::get().to(Self::fetch))
    }
}
