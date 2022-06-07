mod database;
mod handler;
mod model;
mod request;
mod setting;
mod store;

use actix_web::{web, App, HttpServer};

use setting::Settings;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    simple_logger::init_with_level(log::Level::Info).expect("logger initiation failed");

    let setting = Settings::new().expect("loading configuration failed");

    let db = database::connect(setting.database()).await;

    HttpServer::new(move || {
        let url_handler = handler::Url::new(store::Url::new(db.clone()));

        App::new()
            .service(url_handler.register(web::scope("/api")))
            .service(handler::Healthz::register(web::scope("/")))
    })
    .workers(12)
    .bind(format!(
        "{}:{}",
        setting.server().host(),
        setting.server().port()
    ))?
    .run()
    .await
}
