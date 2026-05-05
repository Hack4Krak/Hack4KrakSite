use sea_orm_migration::prelude::extension::postgres::Type;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(FoodPreference::Enum)
                    .values([FoodPreference::Standard, FoodPreference::Vegetarian])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(EventRegistration::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(EventRegistration::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(EventRegistration::UserId)
                            .uuid()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(EventRegistration::FullName).string().not_null())
                    .col(ColumnDef::new(EventRegistration::School).string().not_null())
                    .col(ColumnDef::new(EventRegistration::BirthYear).string().not_null())
                    .col(ColumnDef::new(EventRegistration::Phone).string().not_null())
                    .col(
                        ColumnDef::new(EventRegistration::IsUnderage)
                            .boolean()
                            .not_null(),
                    )
                    .col(ColumnDef::new(EventRegistration::EmergencyContactName).string())
                    .col(ColumnDef::new(EventRegistration::EmergencyContactPhone).string())
                    .col(ColumnDef::new(EventRegistration::EmergencyContactEmail).string())
                    .col(
                        ColumnDef::new(EventRegistration::FoodPreference)
                            .custom(FoodPreference::Enum)
                            .not_null(),
                    )
                    .col(ColumnDef::new(EventRegistration::FoodAllergies).string())
                    .col(
                        ColumnDef::new(EventRegistration::RegisteredAt)
                            .date_time()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-event_registration-user_id")
                            .from(EventRegistration::Table, EventRegistration::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(EventRegistration::Table).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(FoodPreference::Enum).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum EventRegistration {
    Table,
    Id,
    UserId,
    FullName,
    School,
    BirthYear,
    Phone,
    IsUnderage,
    EmergencyContactName,
    EmergencyContactPhone,
    EmergencyContactEmail,
    FoodPreference,
    FoodAllergies,
    RegisteredAt,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}

#[derive(DeriveIden)]
#[sea_orm(enum_name = "food_preference")]
enum FoodPreference {
    #[sea_orm(iden = "food_preference")]
    Enum,
    #[sea_orm(iden = "standard")]
    Standard,
    #[sea_orm(iden = "vegetarian")]
    Vegetarian,
}
