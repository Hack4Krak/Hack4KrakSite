use hack4krak_backend::utils::error::Error;
use sea_orm::DbErr;

#[test]
fn db_custom_error_maps_to_database_operation() {
    let db_err = DbErr::Custom("something went wrong".to_string());
    let error = Error::from(db_err);
    assert!(matches!(error, Error::DatabaseOperation(_)));
}

#[test]
fn db_record_not_found_maps_to_database_operation() {
    let db_err = DbErr::RecordNotFound("not found".to_string());
    let error = Error::from(db_err);
    assert!(matches!(error, Error::DatabaseOperation(_)));
}

#[test]
fn db_conn_acquire_error_maps_to_database_operation() {
    let db_err = DbErr::ConnectionAcquire(sea_orm::ConnAcquireErr::Timeout);
    let error = Error::from(db_err);
    assert!(matches!(error, Error::DatabaseOperation(_)));
}
