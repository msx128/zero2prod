use actix_web::{HttpResponse, web};

#[derive(serde::Deserialize, Debug)]
pub struct MemoryUsageInfo {
    timestamp: u64,
    used: u64,
    total: u64,
}

#[tracing::instrument(
    name = "Getting memory usage info",
    fields(date = %form.timestamp, used = %form.used, total = %form.total)
)]
pub async fn mem_info(
    form: web::Json<MemoryUsageInfo>,
) -> HttpResponse {
    HttpResponse::Ok().finish()
}
