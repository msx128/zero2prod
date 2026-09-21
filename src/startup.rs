use crate::routes::{
    cpu_info, disk_info, health_check, mem_info, network_info,
    subscribe,
};
use actix_web::dev::Server;
use actix_web::web::{self, Data};
use actix_web::{App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub fn run(
    listener: TcpListener,
    db_pool: PgPool,
) -> Result<Server, std::io::Error> {
    let db_pool = Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscribtions", web::post().to(subscribe))
            .route("/api/v1/cpu_info", web::post().to(cpu_info))
            .route("/api/v1/mem_info", web::post().to(mem_info))
            .route("/api/v1/disk_info", web::post().to(disk_info))
            .route(
                "/api/v1/network_info",
                web::post().to(network_info),
            )
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
