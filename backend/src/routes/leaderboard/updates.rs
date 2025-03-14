use crate::utils::app_state;
use crate::utils::sse_event::SseEvent;
use actix_web::{HttpResponse, Responder, web};

pub async fn sse_handler(app_state: web::Data<app_state::AppState>) -> impl Responder {
    let tx = &app_state.sse_event_sender;
    let rx = tx.subscribe();

    let server_events = SseEvent::create_event_stream(rx);

    HttpResponse::Ok()
        .content_type("text/event-stream")
        .streaming(server_events)
}
