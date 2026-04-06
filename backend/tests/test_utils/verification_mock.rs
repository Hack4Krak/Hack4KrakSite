use async_trait::async_trait;
use hack4krak_backend::services::verification::VerificationQrCodeSender;
use hack4krak_backend::utils::app_state::AppState;
use hack4krak_backend::utils::error::Error;
use uuid::Uuid;

mockall::mock! {
    pub VerificationEmailSender {}

    #[async_trait]
    impl VerificationQrCodeSender for VerificationEmailSender {
        async fn send_verification_qr_email(
            &self,
            app_state: &AppState,
            username: &str,
            email: &str,
            verification_id: Uuid,
        ) -> Result<(), Error>;
    }
}
