use crate::utils::error::Error;
use sea_orm::ActiveModelTrait;

pub fn handle_database_error<T>(
    result: Result<T, sea_orm::DbErr>,
    returned_error: Error,
) -> Result<(), Error>
where
    T: ActiveModelTrait,
{
    match result.map_err(Error::from) {
        Ok(_) => Ok(()),
        Err(Error::ConflictInDatabase) => Err(returned_error),
        Err(error) => Err(error),
    }
}
