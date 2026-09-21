use actix_web::{HttpResponse, web};

#[derive(serde::Deserialize, Debug)]
pub struct NetworkInfo {
    timestamp: u64,
    name: String,
    received: u64,
    transmited: u64,
}

#[tracing::instrument(
    name = "Getting network info",
    fields(
        date = %form.timestamp,
        name = %form.name,
        recieved = %form.received,
        transmited = %form.transmited,
    )
)]
pub async fn network_info(
    form: web::Json<NetworkInfo>,
) -> HttpResponse {
    HttpResponse::Ok().finish()
}
