use crate::derive_configure;
use crate::models::requests::AddDownloadTaskRequest;
use crate::models::responses::AddDownloadTaskResponse;
use actix_web::{post, web};

use crate::Result;
/// endpoint for adding a new download task
/// takes `AddDownloadTaskRequest` as request body
#[post("/")]
async fn add_task(
    req: web::Json<AddDownloadTaskRequest>,
    state: web::Data<crate::state::AppState<'_>>,
) -> Result<AddDownloadTaskResponse> {
    let resp = state.downloader.add_task(req.url.as_str()).await?;
    Ok(resp)
}

derive_configure!("/download", add_task);
