use crate::entities::{flag_capture, teams};
use crate::routes::flag::FlagError;
use crate::routes::task::TaskError;
use crate::utils::error::Error;
use chrono::Local;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, TryInsertResult,
    sea_query,
};

impl flag_capture::Model {
    pub async fn completed(
        database: &DatabaseConnection,
        team: teams::Model,
        task_id: String,
    ) -> Result<bool, Error> {
        let result = flag_capture::Entity::insert(flag_capture::ActiveModel {
            team: Set(team.id),
            task: Set(task_id.clone()),
            submitted_at: Set(Local::now().naive_local()),
            ..Default::default()
        })
        .on_conflict(
            sea_query::OnConflict::columns(vec![
                flag_capture::Column::Team,
                flag_capture::Column::Task,
            ])
            .do_nothing()
            .to_owned(),
        )
        .do_nothing()
        .exec(database)
        .await?;

        match result {
            TryInsertResult::Inserted(_) => {
                Ok(Self::is_first_flag_submission(database, task_id).await?)
            }
            _ => Err(Error::Flag(FlagError::AlreadySubmittedFlag)),
        }
    }

    pub async fn is_first_flag_submission(
        database: &DatabaseConnection,
        task_id: String,
    ) -> Result<bool, Error> {
        match flag_capture::Entity::find()
            .filter(flag_capture::Column::Task.eq(task_id))
            .count(database)
            .await
            .map_err(|_| Error::Task(TaskError::InvalidTaskId))?
        {
            1 => Ok(true),
            _ => Ok(false),
        }
    }
}
