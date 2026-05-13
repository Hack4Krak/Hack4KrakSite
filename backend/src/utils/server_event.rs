use crate::entities::users;
use crate::utils::app_state::AppState;
use crate::utils::cookies::ACCESS_TOKEN_COOKIE;
use crate::utils::jwt::decode_jwt;
use actix_web::{Error, HttpRequest, web::Bytes};
use async_stream::__private::AsyncStream;
use async_stream::stream;
use sea_orm::EntityTrait;
use sea_orm::prelude::Uuid;
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use std::collections::HashSet;
use std::time::Duration;
use tokio::sync::broadcast::{Receiver, error::RecvError};
use tokio::time::interval;
use utoipa::ToSchema;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(30);

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum ServerEventTopic {
    Leaderboard,
    Team,
}

#[derive(Debug, Clone, Copy)]
pub enum EventAudience {
    Public,
    Team(Uuid),
}

#[derive(Debug, Clone)]
pub struct SubscriberFilter {
    pub topics: HashSet<ServerEventTopic>,
    pub team_id: Option<Uuid>,
}

impl SubscriberFilter {
    pub async fn from_request(
        request: &HttpRequest,
        app_state: &AppState,
        topics: HashSet<ServerEventTopic>,
    ) -> Self {
        let team_id = Self::extract_team_id(request, app_state).await;
        Self { topics, team_id }
    }

    async fn extract_team_id(request: &HttpRequest, app_state: &AppState) -> Option<Uuid> {
        let cookie = request.cookie(ACCESS_TOKEN_COOKIE)?;
        let claims = decode_jwt(cookie.value()).ok()?.claims;
        let user = users::Entity::find_by_id(claims.id)
            .one(&app_state.database)
            .await
            .ok()??;
        user.team
    }

    pub fn accepts(&self, envelope: &BroadcastEnvelope) -> bool {
        if !self.topics.contains(&envelope.event.topic()) {
            return false;
        }

        match envelope.audience {
            EventAudience::Public => true,
            EventAudience::Team(team_id) => self.team_id == Some(team_id),
        }
    }
}

#[derive(Debug, Serialize, Clone, ToSchema)]
pub struct LeaderboardUpdateEvent {
    pub task_id: String,
    pub task_name: String,
    pub is_first_flag_submission: bool,
    pub team_name: String,
    pub username: String,
}

#[derive(Debug, Serialize, Clone, ToSchema)]
pub struct TeamFlagCaptureEvent {
    pub task_id: String,
    pub task_name: String,
    pub username: String,
    pub is_first_flag_submission: bool,
}

#[derive(Debug, Serialize, Clone, ToSchema)]
#[serde(tag = "type", content = "payload", rename_all = "snake_case")]
pub enum ServerEvent {
    LeaderboardUpdate(LeaderboardUpdateEvent),
    TeamFlagCapture(TeamFlagCaptureEvent),
}

#[derive(Debug, Clone)]
pub struct BroadcastEnvelope {
    pub event: ServerEvent,
    pub audience: EventAudience,
}

impl BroadcastEnvelope {
    pub fn public(event: ServerEvent) -> Self {
        Self {
            event,
            audience: EventAudience::Public,
        }
    }

    pub fn team(team_id: Uuid, event: ServerEvent) -> Self {
        Self {
            event,
            audience: EventAudience::Team(team_id),
        }
    }
}

impl ServerEvent {
    pub fn topic(&self) -> ServerEventTopic {
        match self {
            ServerEvent::LeaderboardUpdate(_) => ServerEventTopic::Leaderboard,
            ServerEvent::TeamFlagCapture(_) => ServerEventTopic::Team,
        }
    }

    fn event_name(&self) -> &'static str {
        match self {
            ServerEvent::LeaderboardUpdate(_) => "leaderboard_update",
            ServerEvent::TeamFlagCapture(_) => "team_flag_capture",
        }
    }

    /// https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events#event_stream_format
    pub fn to_message(&self) -> Result<Bytes, Error> {
        let data = format!(
            "event: {}\ndata: {}\n\n",
            self.event_name(),
            to_string(&self)?
        );
        Ok(Bytes::from(data))
    }

    pub fn heartbeat_message() -> Bytes {
        Bytes::from_static(b": heartbeat\n\n")
    }

    pub fn create_event_stream(
        mut receiver: Receiver<BroadcastEnvelope>,
        filter: SubscriberFilter,
    ) -> AsyncStream<Result<Bytes, Error>, impl Future<Output = ()>> {
        stream! {
            let mut heartbeat = interval(HEARTBEAT_INTERVAL);

            loop {
                tokio::select! {
                    _ = heartbeat.tick() => {
                        yield Ok(ServerEvent::heartbeat_message());
                    }
                    event = receiver.recv() => {
                        match event {
                            Ok(envelope) if filter.accepts(&envelope) => {
                                yield envelope.event.to_message();
                            }
                            Ok(_) | Err(RecvError::Lagged(_)) => {}
                            Err(RecvError::Closed) => break,
                        }
                    }
                }
            }
        }
    }
}
