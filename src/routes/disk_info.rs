use actix_web::{HttpResponse, web};

#[allow(dead_code)]
#[derive(serde::Deserialize, Debug)]
pub struct DiskInfo {
    timestamp: u64,
    kind: String,
    name: String,
    total: u64,
    available: u64,
    used: u64,
}

#[derive(serde::Deserialize, Debug)]
pub struct DisksInfo {
    disks: Vec<DiskInfo>,
}

#[tracing::instrument(
    name = "Getting disk info",
    fields(
        disks = ?form.disks,
    )
)]
pub async fn disk_info(form: web::Json<DisksInfo>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
