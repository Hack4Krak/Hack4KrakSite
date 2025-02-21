use crate::entities::{flag_capture, teams};
use crate::routes::flag::FlagError;
use crate::utils::error::Error;
use chrono::Local;
use sea_orm::ActiveValue::Set;
use sea_orm::{sea_query, DatabaseConnection, EntityTrait, TryInsertResult};

impl flag_capture::Model {
    pub async fn completed(
        database: &DatabaseConnection,
        team: teams::Model,
        task_id: String,
    ) -> Result<(), Error> {
        let result = flag_capture::Entity::insert(flag_capture::ActiveModel {
            team: Set(team.id),
            task: Set(task_id),
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
            TryInsertResult::Inserted(_) => Ok(()),
            _ => Err(Error::Flag(FlagError::AlreadySubmittedFlag)),
        }
    }
}
