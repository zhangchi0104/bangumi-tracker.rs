mod bangumi;
mod download;
mod torrent;
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .configure(bangumi::configure)
            .configure(torrent::configure)
            .configure(download::configure),
    );
}
