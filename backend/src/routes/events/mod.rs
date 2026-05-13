use crate::utils::app_state::AppState;
use crate::utils::server_event::{ServerEvent, ServerEventTopic, SubscriberFilter};
use actix_web::{HttpRequest, HttpResponse, Responder, get, web};
use serde::Deserialize;
use std::collections::HashSet;
use utoipa::IntoParams;

#[derive(Debug, Deserialize, IntoParams)]
struct EventStreamQuery {
    #[serde(default)]
    topics: Vec<ServerEventTopic>,
}

#[utoipa::path(
    params(EventStreamQuery),
    responses(
        (status = 200, description = "Server-sent event stream for selected topics. Authentication is optional: when present, team-scoped events are forwarded to the authenticated user's team. Heartbeat comments are emitted every 30 seconds.", content_type = "text/event-stream", body = ServerEvent)
    ),
    tag = "events"
)]
#[get("/stream")]
pub async fn stream(
    app_state: web::Data<AppState>,
    request: HttpRequest,
    query: web::Query<EventStreamQuery>,
) -> impl Responder {
    let topics = query.topics.iter().copied().collect::<HashSet<_>>();
    let filter = SubscriberFilter::from_request(&request, &app_state, topics).await;
    let events =
        ServerEvent::create_event_stream(app_state.server_event_sender.subscribe(), filter);

    HttpResponse::Ok()
        .content_type("text/event-stream")
        .append_header(("Cache-Control", "no-cache, no-transform"))
        .append_header(("Connection", "keep-alive"))
        .append_header(("X-Accel-Buffering", "no"))
        .streaming(events)
}

pub fn config(config: &mut utoipa_actix_web::service_config::ServiceConfig) {
    config.service(stream);
}
