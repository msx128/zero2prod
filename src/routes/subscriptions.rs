use actix_web::{HttpResponse, web};
use chrono::Utc;
use sqlx::PgPool;
use tracing::Instrument;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct SubscribBundle {
    email: String,
    name: String,
}

pub async fn subscribe(
    form: web::Form<SubscribBundle>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let request_id = Uuid::new_v4();
    let request_span = tracing::info_span!(
        "Adding a new subscriber.",
        subscriber_email = %form.email,
        subscriber_name = %form.name,
    );
    let _request_span_guard = request_span.enter();
    tracing::info!(
        "request_id {} - Adding '{}' '{}' as a new subscriber.\nSaving details in the database",
        request_id,
        form.email,
        form.name,
    );
    let query_span = tracing::info_span!(
        "Saving new subscriber details in the database"
    );
    match sqlx::query!(
        r#"
            insert into subscriptions (id, email, name, subscribed_at)
            values ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool.as_ref())
    .instrument(query_span)
    .await
    {
        Ok(_) => {
            tracing::info!(
                "request_id {} - New subscriber details have been saved",
                request_id,
            );
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!(
                "request_id {} - Failed to execute query: {:?}",
                request_id,
                e,
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}
