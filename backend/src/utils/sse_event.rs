use actix_web::Error;
use actix_web::web::Bytes;
use async_stream::__private::AsyncStream;
use async_stream::stream;
use serde::Serialize;
use serde_json::to_string;
use tokio::sync::broadcast::Receiver;

#[derive(Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum SseEvent {
    LeaderboardUpdate {
        task_id: String,
        task_name: String,
        is_first_flag_submission: bool,
        team_name: String,
        username: String,
    },
}

impl SseEvent {
    pub fn to_message(&self) -> Result<Bytes, Error> {
        // Due to server-sent events requirements, sent data should be formated with
        // "data: " prefix and two newlines at the end
        // https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events#event_stream_format
        let data = format!("data: {}\n\n", to_string(&self)?);
        Ok(Bytes::from(data))
    }

    pub fn create_event_stream(
        mut receiver: Receiver<SseEvent>,
    ) -> AsyncStream<Result<Bytes, Error>, impl Future<Output = ()>> {
        stream! {
            while let Ok(message) = receiver.recv().await {
                yield message.to_message();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_to_message() {
        use crate::utils::sse_event::SseEvent;
        use actix_web::web::Bytes;

        let event = SseEvent::LeaderboardUpdate {
            task_id: "task_id".to_string(),
            task_name: "task_name".to_string(),
            is_first_flag_submission: true,
            team_name: "team_name".to_string(),
            username: "username".to_string(),
        };

        let message = event.to_message().unwrap();
        let expected_message = "data: {\"task_id\":\"task_id\",\"task_name\":\"task_name\",\"is_first_flag_submission\":true,\"team_name\":\"team_name\",\"username\":\"username\"}\n\n";
        assert_eq!(message, Bytes::from(expected_message));
    }
}
