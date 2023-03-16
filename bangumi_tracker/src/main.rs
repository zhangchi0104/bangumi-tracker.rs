use actix_web::{middleware::Logger, web, App, HttpServer};
use env_logger::Env;

use errors::Error;

pub mod downloaders;
pub mod errors;
pub mod models;
pub mod routes;
pub mod sources;
pub mod state;

pub type Result<T> = std::result::Result<T, Error>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .configure(routes::configure)
            .app_data(web::Data::new(state::AppState::new()))
    })
    .bind(("127.0.0.1", 8080))
    .expect("Unable to start server")
    .run()
    .await
}
