use actix_web::{HttpResponse, Responder, Scope, get};

#[get("/healthz")]
async fn healthz() -> impl Responder {
    HttpResponse::NoContent()
}

pub fn register(scope: Scope) -> Scope {
    scope.service(healthz)
}
