use actix_web::{middleware::Logger, web, App, HttpServer};
use env_logger::Env;

use errors::Error;

pub mod downloaders;
pub mod errors;
pub mod models;
pub mod routes;
pub mod sources;
pub mod state;
pub(crate) mod utils;
pub use utils::*;
pub type Result<T, E = Error> = std::result::Result<T, E>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .configure(routes::configure)
            .app_data(web::Data::new(state::AppState::from_env_or_default()))
    })
    .bind(("127.0.0.1", 8080))
    .expect("Unable to start server: ")
    .run()
    .await
}
