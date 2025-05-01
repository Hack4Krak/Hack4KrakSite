use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::{date_time, json_binary_null, pk_uuid, string, timestamp_null};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(EmailConfirmation::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(
                Table::drop()
                    .table(PasswordReset::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(EmailVerificationRequest::Table)
                    .if_not_exists()
                    .col(pk_uuid(FlagCapture::Id).extra("DEFAULT gen_random_uuid()".to_string()))
                    .col(string(EmailVerificationRequest::Email))
                    .col(string(EmailVerificationRequest::ActionType).char_len(250))
                    .col(timestamp_null(EmailVerificationRequest::ExpirationTime))
                    .col(json_binary_null(EmailVerificationRequest::AdditionalData))
                    .col(
                        date_time(EmailVerificationRequest::CreatedAt)
                            .not_null()
                            .extra("DEFAULT NOW()".to_string()),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop the `flag_capture` table
        manager
            .drop_table(Table::drop().table(FlagCapture::Table).to_owned())
            .await?;

        let db = manager.get_connection();
        db.execute_unprepared(r#"
            CREATE TABLE public.password_reset (
                code uuid NOT NULL,
                expiration_date timestamp without time zone NOT NULL,
                "user" uuid NOT NULL
            );
            ALTER TABLE ONLY public.password_reset ADD CONSTRAINT password_reset_pkey PRIMARY KEY (code);
            ALTER TABLE ONLY public.password_reset ADD CONSTRAINT password_reset_user FOREIGN KEY ("user") REFERENCES public.users(id) ON DELETE CASCADE;

            CREATE TABLE public.email_confirmation (
                email character varying NOT NULL,
                user_info json NOT NULL,
                code character varying NOT NULL,
                expiration_date timestamp without time zone NOT NULL
            );
            ALTER TABLE ONLY public.email_confirmation ADD CONSTRAINT email_confirmation_pkey PRIMARY KEY (code);
        "#).await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum FlagCapture {
    Table,
    Id,
}

#[derive(DeriveIden)]
pub enum EmailVerificationRequest {
    Table,
    Email,
    ActionType,
    ExpirationTime,
    AdditionalData,
    CreatedAt,
}

#[derive(DeriveIden)]
enum PasswordReset {
    Table,
}

#[derive(DeriveIden)]
enum EmailConfirmation {
    Table,
}
