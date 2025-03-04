use crate::utils::app_state;
use actix_web::web::Bytes;
use actix_web::{HttpResponse, Responder, web};
use async_stream::stream;

pub async fn sse_handler(app_state: web::Data<app_state::AppState>) -> impl Responder {
    let tx = &app_state.leaderboard_updates_transmitter;
    let mut rx = tx.subscribe();

    let server_events = stream! {
        while let Ok(msg) = rx.recv().await {
            let data = format!("{}\n\n", msg);
            yield Ok::<Bytes, actix_web::Error>(web::Bytes::from(data));
        }
    };

    HttpResponse::Ok()
        .content_type("text/event-stream")
        .streaming(server_events)
}
