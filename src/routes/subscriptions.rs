use actix_web::{HttpResponse, web};
#[derive(serde::Deserialize)]
pub struct SubscribBundle {
    email: String,
    name: String,
}
pub async fn subscribe(_form: web::Form<SubscribBundle>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
