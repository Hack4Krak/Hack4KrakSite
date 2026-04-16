use crate::test_utils::TestApp;
use actix_http::body::MessageBody;
use actix_web::error::ResponseError;
use actix_web::http::StatusCode;
use actix_web::test::{self, read_body_json};
use hack4krak_backend::entities::sea_orm_active_enums::UserRoles;
use hack4krak_backend::routes::account::AccountError;
use hack4krak_backend::routes::auth::AuthError;
use hack4krak_backend::routes::flag::FlagError;
use hack4krak_backend::routes::task::TaskError;
use hack4krak_backend::routes::teams::TeamError;
use hack4krak_backend::utils::error::Error;
use serde_json::json;

async fn assert_correct_error_format<B: MessageBody + 'static>(
    response: actix_web::dev::ServiceResponse<B>,
) {
    assert_eq!(response.status(), 400);
    let body: serde_json::Value = read_body_json(response).await;
    assert!(
        body.get("message").is_some(),
        "Response should contain 'message' field"
    );
    assert!(
        body.get("code").is_some(),
        "Response should contain 'code' field"
    );
    assert!(
        body.get("error").is_some(),
        "Response should contain 'error' field"
    );
}


#[actix_web::test]
async fn correct_uuid_error_format() {
    let app = TestApp::default().build_app().await;

    let request = test::TestRequest::get()
        .uri("/auth/confirm/not-a-valid-uuid")
        .to_request();

    let response = test::call_service(&app, request).await;
    assert_correct_error_format(response).await;
}


#[actix_web::test]
async fn correct_json_error_format() {
    let app = TestApp::default().build_app().await;

    let request = test::TestRequest::post()
        .uri("/auth/register")
        .insert_header(("Content-Type", "application/json"))
        .set_payload(r#"{"email": "test@example.com", "name": invalid_json}"#)
        .to_request();

    let response = test::call_service(&app, request).await;
    assert_correct_error_format(response).await;
}


#[actix_web::test]
async fn correct_validation_error_format() {
    let app = TestApp::default().build_app().await;

    let request = test::TestRequest::post()
        .uri("/auth/register")
        .set_json(json!({
            "email": "this_!isn'taemaill",
            "name": "test_user",
            "password": "password123"
        }))
        .to_request();

    let response = test::call_service(&app, request).await;
    assert_correct_error_format(response).await;
}

#[test]
fn unauthorized_returns_401() {
    assert_eq!(Error::Unauthorized.status_code(), StatusCode::UNAUTHORIZED);
}

#[test]
fn invalid_jwt_returns_401() {
    assert_eq!(
        Error::InvalidJsonWebToken.status_code(),
        StatusCode::UNAUTHORIZED
    );
}

#[test]
fn forbidden_returns_403() {
    assert_eq!(
        Error::Forbidden {
            required_role: UserRoles::Admin
        }
        .status_code(),
        StatusCode::FORBIDDEN
    );
}

#[test]
fn user_not_found_returns_404() {
    assert_eq!(Error::UserNotFound.status_code(), StatusCode::NOT_FOUND);
}

#[test]
fn route_not_found_returns_404() {
    assert_eq!(Error::RouteNotFound.status_code(), StatusCode::NOT_FOUND);
}

#[test]
fn recipient_not_found_returns_404() {
    assert_eq!(
        Error::RecipientNotFound {
            username: "test".to_string()
        }
        .status_code(),
        StatusCode::NOT_FOUND
    );
}

#[test]
fn invalid_uuid_returns_400() {
    assert_eq!(Error::InvalidUuid.status_code(), StatusCode::BAD_REQUEST);
}

#[test]
fn json_deserialization_error_returns_400() {
    assert_eq!(
        Error::JsonDeserializationError.status_code(),
        StatusCode::BAD_REQUEST
    );
}

#[test]
fn placeholders_required_returns_400() {
    assert_eq!(
        Error::PlaceholdersRequired.status_code(),
        StatusCode::BAD_REQUEST
    );
}

#[test]
fn missing_extension_returns_400() {
    assert_eq!(
        Error::MissingExtension {
            name: "test".to_string()
        }
        .status_code(),
        StatusCode::BAD_REQUEST
    );
}

#[test]
fn invalid_email_confirmation_code_returns_400() {
    assert_eq!(
        Error::InvalidEmailConfirmationCode.status_code(),
        StatusCode::BAD_REQUEST
    );
}

#[test]
fn email_confirmation_code_expired_returns_400() {
    assert_eq!(
        Error::EmailConfirmationCodeExpired.status_code(),
        StatusCode::BAD_REQUEST
    );
}

#[test]
fn user_with_email_or_username_already_exists_returns_409() {
    assert_eq!(
        Error::UserWithEmailOrUsernameAlreadyExists.status_code(),
        StatusCode::CONFLICT
    );
}

#[test]
fn access_before_stage_returns_403() {
    assert_eq!(
        Error::AccessBeforeStage {
            stage_start_date: "2025-01-01".to_string()
        }
        .status_code(),
        StatusCode::FORBIDDEN
    );
}

#[test]
fn access_after_stage_returns_410() {
    assert_eq!(
        Error::AccessAfterStage {
            stage_end_date: "2025-01-01".to_string()
        }
        .status_code(),
        StatusCode::GONE
    );
}

#[test]
fn access_during_stage_returns_403() {
    assert_eq!(
        Error::AccessDuringStage.status_code(),
        StatusCode::FORBIDDEN
    );
}

#[test]
fn failed_to_parse_stage_returns_500() {
    assert_eq!(
        Error::FailedToParseStage {
            stage_identifier: "test".to_string()
        }
        .status_code(),
        StatusCode::INTERNAL_SERVER_ERROR
    );
}

#[test]
fn user_must_have_higher_role_returns_403() {
    assert_eq!(
        Error::UserMustHaveHigherRoleThanAffectedUser.status_code(),
        StatusCode::FORBIDDEN
    );
}

#[test]
fn user_must_be_owner_to_update_roles_returns_403() {
    assert_eq!(
        Error::UserMustBeOwnerToUpdateRoles.status_code(),
        StatusCode::FORBIDDEN
    );
}

#[test]
fn conflict_in_database_returns_500() {
    assert_eq!(
        Error::ConflictInDatabase.status_code(),
        StatusCode::INTERNAL_SERVER_ERROR
    );
}

#[test]
fn invalid_color_format_returns_400() {
    assert_eq!(
        Error::InvalidColorFormat.status_code(),
        StatusCode::BAD_REQUEST
    );
}

#[test]
fn auth_error_user_already_exists_returns_409() {
    assert_eq!(
        AuthError::UserAlreadyExists.status_code(),
        StatusCode::CONFLICT
    );
}

#[test]
fn auth_error_invalid_credentials_returns_401() {
    assert_eq!(
        AuthError::InvalidCredentials.status_code(),
        StatusCode::UNAUTHORIZED
    );
}

#[test]
fn auth_error_invalid_email_returns_401() {
    assert_eq!(
        AuthError::InvalidEmailAddress.status_code(),
        StatusCode::UNAUTHORIZED
    );
}

#[test]
fn auth_error_password_auth_not_available_returns_401() {
    assert_eq!(
        AuthError::PasswordAuthNotAvailable.status_code(),
        StatusCode::UNAUTHORIZED
    );
}

#[test]
fn team_error_already_exists_returns_409() {
    assert_eq!(
        TeamError::AlreadyExists {
            team_name: "test".to_string()
        }
        .status_code(),
        StatusCode::CONFLICT
    );
}

#[test]
fn team_error_not_found_returns_404() {
    assert_eq!(TeamError::TeamNotFound.status_code(), StatusCode::NOT_FOUND);
}

#[test]
fn team_error_user_doesnt_belong_returns_403() {
    assert_eq!(
        TeamError::UserDoesntBelongToAnyTeam {
            username: "test".to_string()
        }
        .status_code(),
        StatusCode::FORBIDDEN
    );
}

#[test]
fn team_error_user_is_not_leader_returns_403() {
    assert_eq!(
        TeamError::UserIsNotTeamLeader.status_code(),
        StatusCode::FORBIDDEN
    );
}

#[test]
fn team_error_team_is_full_returns_409() {
    assert_eq!(
        TeamError::TeamIsFull { max_size: 5 }.status_code(),
        StatusCode::CONFLICT
    );
}

#[test]
fn team_error_invalid_registration_period_returns_400() {
    assert_eq!(
        TeamError::InvalidRegistrationPeriod.status_code(),
        StatusCode::BAD_REQUEST
    );
}

#[test]
fn flag_error_invalid_format_returns_400() {
    assert_eq!(
        FlagError::InvalidFlagFormat.status_code(),
        StatusCode::BAD_REQUEST
    );
}

#[test]
fn flag_error_invalid_flag_returns_400() {
    assert_eq!(
        FlagError::InvalidFlag.status_code(),
        StatusCode::BAD_REQUEST
    );
}

#[test]
fn flag_error_already_submitted_returns_409() {
    assert_eq!(
        FlagError::AlreadySubmittedFlag.status_code(),
        StatusCode::CONFLICT
    );
}

#[test]
fn flag_error_team_not_confirmed_returns_403() {
    assert_eq!(
        FlagError::TeamNotConfirmed.status_code(),
        StatusCode::FORBIDDEN
    );
}

#[test]
fn account_error_birth_year_not_in_range_returns_400() {
    assert_eq!(
        AccountError::BirthYearNotInRange {
            min: 1900,
            max: 2025
        }
        .status_code(),
        StatusCode::BAD_REQUEST
    );
}

#[test]
fn account_error_invalid_referral_source_returns_400() {
    assert_eq!(
        AccountError::InvalidReferralSource.status_code(),
        StatusCode::BAD_REQUEST
    );
}

#[test]
fn task_error_invalid_id_returns_400() {
    assert_eq!(
        TaskError::InvalidTaskId.status_code(),
        StatusCode::BAD_REQUEST
    );
}

#[test]
fn task_error_missing_task_returns_404() {
    assert_eq!(
        TaskError::MissingTask {
            id: "test".to_string()
        }
        .status_code(),
        StatusCode::NOT_FOUND
    );
}

#[test]
fn task_error_could_not_load_asset_returns_500() {
    assert_eq!(
        TaskError::CouldNotLoadTaskAsset {
            id: "test".to_string()
        }
        .status_code(),
        StatusCode::INTERNAL_SERVER_ERROR
    );
}

#[test]
fn error_response_contains_correct_status() {
    let error = Error::Unauthorized;
    let response = error.error_response();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[test]
fn forbidden_error_response_has_correct_status() {
    let error = Error::Forbidden {
        required_role: UserRoles::Admin,
    };
    let response = error.error_response();
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

#[test]
fn nested_team_error_maps_to_correct_status() {
    let error: Error = TeamError::TeamNotFound.into();
    assert_eq!(error.status_code(), StatusCode::NOT_FOUND);
}

#[test]
fn nested_auth_error_maps_to_correct_status() {
    let error: Error = AuthError::InvalidCredentials.into();
    assert_eq!(error.status_code(), StatusCode::UNAUTHORIZED);
}

#[test]
fn nested_flag_error_maps_to_correct_status() {
    let error: Error = FlagError::AlreadySubmittedFlag.into();
    assert_eq!(error.status_code(), StatusCode::CONFLICT);
}

#[test]
fn nested_account_error_maps_to_correct_status() {
    let error: Error = AccountError::InvalidReferralSource.into();
    assert_eq!(error.status_code(), StatusCode::BAD_REQUEST);
}

#[test]
fn nested_task_error_maps_to_correct_status() {
    let error: Error = TaskError::InvalidTaskId.into();
    assert_eq!(error.status_code(), StatusCode::BAD_REQUEST);
}
