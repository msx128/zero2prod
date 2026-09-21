use actix_web::{HttpResponse, web};

#[derive(serde::Deserialize, Debug)]
pub struct CpuUsageInfo {
    timestamp: u64,
    cpu_usage: Vec<f32>,
}

#[tracing::instrument(
    name = "Getting cpu usage info",
    fields(date = %form.timestamp, info = ?form.cpu_usage)
)]
pub async fn cpu_info(form: web::Json<CpuUsageInfo>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
