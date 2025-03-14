use actix_web::Error;
use actix_web::web::Bytes;
use async_stream::__private::AsyncStream;
use async_stream::stream;
use serde::Serialize;
use tokio::sync::broadcast::Receiver;

#[derive(Serialize)]
pub enum SseMessage {
    LeaderboardUpdate {
        task_id: String,
        task_name: String,
        is_first_flag_submission: bool,
        team_name: String,
        username: String,
    },
}

impl SseMessage {
    pub fn create_event_stream(
        mut rx: Receiver<String>,
    ) -> AsyncStream<Result<Bytes, Error>, impl Future<Output = ()>> {
        stream! {
            while let Ok(msg) = rx.recv().await {
                // Due to server-sent events requirements, sent data should be formated with
                // "data: " prefix and two newlines at the end
                // https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events#event_stream_format
                let data = format!("data: {}\n\n", msg);
                yield Ok::<Bytes, actix_web::Error>(Bytes::from(data));
            }
        }
    }
}
