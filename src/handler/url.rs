use actix_web::{web, HttpResponse, Responder, Scope};

use crate::request;
use crate::model;

pub struct URL {
}

impl URL {
    async fn create(url: web::Json<request::URL>) -> impl Responder {
        println!("get {:?}", url);
        HttpResponse::Ok().json(model::URL::new(url.url(), url.name()))
    }

    pub fn register(scope: Scope) -> Scope {
        scope
            .route("/url", web::post().to(Self::create))
    }
}
