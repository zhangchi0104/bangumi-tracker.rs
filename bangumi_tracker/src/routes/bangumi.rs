use actix_web::{get, Responder, web};

use crate::{Result, state::AppState};

#[get("/current")]
async fn current_bangumi(state: web::Data<AppState>) -> Result<impl Responder> {
    state.bangumi_source.fetch_current_season_bangumi().await
}

pub(super) fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/bangumi").service(current_bangumi));
}
