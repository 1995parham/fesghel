mod handler;
mod model;
mod request;
mod setting;
mod store;

use actix_web::{web, App, HttpServer};

use setting::Settings;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let setting = Settings::new().expect("loading configuration failed");

    HttpServer::new(|| {
        App::new()
            .service(
                handler::url::URL::register(web::scope("/api"))
            )
            .service(
                handler::healthz::Healthz::register(web::scope("/"))
            )
    })
    .bind(
        format!("{}:{}", setting.server().host(), setting.server().port())
        )?
    .run()
    .await
}
