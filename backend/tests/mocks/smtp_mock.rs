use hack4krak_backend::utils::email::SmtpClient;
use hack4krak_backend::utils::error::Error;
use lettre::Message;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Clone, Default)]
pub struct MockSmtpClient {
    send_count: Arc<AtomicUsize>,
}

impl MockSmtpClient {
    pub fn send_count(&self) -> usize {
        self.send_count.load(Ordering::Relaxed)
    }
}

impl SmtpClient for MockSmtpClient {
    fn send(&self, _email: &Message) -> Result<(), Error> {
        self.send_count.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }
}
