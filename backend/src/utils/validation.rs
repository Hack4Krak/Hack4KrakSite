use validator::ValidationError;

pub fn validate_callback(callback: &str) -> Result<(), ValidationError> {
    if callback.starts_with('/') {
        return Ok(());
    }
    Err(ValidationError::new("invalid_callback")
        .with_message("Callback URL must start with '/'".into()))
}
