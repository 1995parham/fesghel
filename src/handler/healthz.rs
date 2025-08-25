use actix_web::{HttpResponse, Responder, Scope, web};

pub struct Healthz {}

impl Healthz {
    async fn healthz() -> impl Responder {
        HttpResponse::NoContent()
    }

    pub fn register(scope: Scope) -> Scope {
        scope.route("/healthz", web::get().to(Self::healthz))
    }
}
