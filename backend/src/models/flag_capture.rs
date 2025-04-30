use crate::entities::{flag_capture, teams};
use crate::routes::flag::FlagError;
use crate::utils::error::Error;
use chrono::Utc;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, TryInsertResult, sea_query,
};

impl flag_capture::Model {
    pub async fn completed(
        database: &DatabaseConnection,
        team: teams::Model,
        task_id: String,
    ) -> Result<bool, Error> {
        let is_first_flag_submission = Self::has_flags_for_task(database, task_id.clone()).await?;

        let result = flag_capture::Entity::insert(flag_capture::ActiveModel {
            team: Set(team.id),
            task: Set(task_id),
            submitted_at: Set(Utc::now().naive_utc()),
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
            TryInsertResult::Inserted(_) => Ok(is_first_flag_submission),
            _ => Err(Error::Flag(FlagError::AlreadySubmittedFlag)),
        }
    }

    pub async fn has_flags_for_task(
        database: &DatabaseConnection,
        task_id: String,
    ) -> Result<bool, Error> {
        Ok(flag_capture::Entity::find()
            .filter(flag_capture::Column::Task.eq(task_id))
            .one(database)
            .await?
            .is_some())
    }
}
