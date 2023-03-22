use actix_web::{get, web, Responder};

use crate::{derive_configure, state::AppState, Result};

#[get("/current")]
async fn current_bangumi(state: web::Data<AppState<'_>>) -> Result<impl Responder> {
    state.bangumi_source.fetch_current_season_bangumi().await
}

derive_configure!("/bangumi", current_bangumi);
