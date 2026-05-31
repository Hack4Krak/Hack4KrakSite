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
                    .as_enum(CtfExperience::Enum)
                    .values([
                        CtfExperience::Never,
                        CtfExperience::Beginner,
                        CtfExperience::Intermediate,
                        CtfExperience::Advanced,
                        CtfExperience::Expert,
                    ])
                    .to_owned(),
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(SchoolGrade::Enum)
                    .values([
                        SchoolGrade::NotStudying,
                        SchoolGrade::PrimarySchool,
                        SchoolGrade::Class1,
                        SchoolGrade::Class2,
                        SchoolGrade::Class3,
                        SchoolGrade::Class4,
                        SchoolGrade::Class5,
                        SchoolGrade::University,
                    ])
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk-user_personal_info-user_personal_info_id")
                    .table(Users::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Users::Table)
                    .rename_column(Users::PersonalInfo, Users::Onboarding)
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared("ALTER TABLE user_personal_info RENAME TO user_onboarding;")
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk-users-onboarding")
                    .from(Users::Table, Users::Onboarding)
                    .to(UserOnboarding::Table, UserOnboarding::Id)
                    .on_delete(ForeignKeyAction::SetNull)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(UserOnboarding::Table)
                    .drop_column(UserOnboarding::FirstName)
                    .drop_column(UserOnboarding::BirthYear)
                    .drop_column(UserOnboarding::IsVegetarian)
                    .add_column(
                        ColumnDef::new(UserOnboarding::CtfExperience)
                            .custom(CtfExperience::Enum)
                            .not_null()
                            .extra("DEFAULT 'never'"),
                    )
                    .add_column(
                        ColumnDef::new(UserOnboarding::SchoolGrade)
                            .custom(SchoolGrade::Enum)
                            .not_null()
                            .extra("DEFAULT 'not_studying'"),
                    )
                    .add_column(
                        ColumnDef::new(UserOnboarding::CollabInterest)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(UserOnboarding::Table)
                    .rename_column(
                        UserOnboarding::ReferralSource,
                        UserOnboarding::ReferralSources,
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
                UPDATE user_onboarding
                SET referral_sources = COALESCE(
                    (
                        SELECT json_agg(
                            CASE source
                                WHEN 'Znajomy' THEN 'Friend'
                                WHEN 'Inne' THEN 'Other'
                                ELSE source
                            END
                        )
                        FROM json_array_elements_text(referral_sources) AS source
                    ),
                    '[]'::json
                )
                WHERE referral_sources IS NOT NULL;
                "#,
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(UserOnboarding::Table)
                    .rename_column(
                        UserOnboarding::ReferralSources,
                        UserOnboarding::ReferralSource,
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(UserOnboarding::Table)
                    .drop_column(UserOnboarding::CtfExperience)
                    .drop_column(UserOnboarding::SchoolGrade)
                    .drop_column(UserOnboarding::CollabInterest)
                    .add_column(
                        ColumnDef::new(UserOnboarding::FirstName)
                            .string()
                            .not_null()
                            .default(""),
                    )
                    .add_column(
                        ColumnDef::new(UserOnboarding::BirthYear)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .add_column(
                        ColumnDef::new(UserOnboarding::IsVegetarian)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
                UPDATE user_onboarding
                SET first_name = users.first_name
                FROM users
                WHERE users.onboarding = user_onboarding.id
                    AND users.first_name IS NOT NULL;
                "#,
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk-users-onboarding")
                    .table(Users::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Users::Table)
                    .rename_column(Users::Onboarding, Users::PersonalInfo)
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared("ALTER TABLE user_onboarding RENAME TO user_personal_info;")
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk-user_personal_info-user_personal_info_id")
                    .from(Users::Table, Users::PersonalInfo)
                    .to(UserPersonalInfo::Table, UserPersonalInfo::Id)
                    .on_delete(ForeignKeyAction::SetNull)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_type(Type::drop().name(SchoolGrade::Enum).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(CtfExperience::Enum).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum UserOnboarding {
    Table,
    Id,
    FirstName,
    BirthYear,
    IsVegetarian,
    CtfExperience,
    SchoolGrade,
    CollabInterest,
    ReferralSource,
    ReferralSources,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    PersonalInfo,
    Onboarding,
}

#[derive(DeriveIden)]
enum UserPersonalInfo {
    Table,
    Id,
}

#[derive(DeriveIden)]
#[sea_orm(enum_name = "ctf_experience")]
enum CtfExperience {
    #[sea_orm(iden = "ctf_experience")]
    Enum,
    #[sea_orm(iden = "never")]
    Never,
    #[sea_orm(iden = "beginner")]
    Beginner,
    #[sea_orm(iden = "intermediate")]
    Intermediate,
    #[sea_orm(iden = "advanced")]
    Advanced,
    #[sea_orm(iden = "expert")]
    Expert,
}

#[derive(DeriveIden)]
#[sea_orm(enum_name = "school_grade")]
enum SchoolGrade {
    #[sea_orm(iden = "school_grade")]
    Enum,
    #[sea_orm(iden = "not_studying")]
    NotStudying,
    #[sea_orm(iden = "primary_school")]
    PrimarySchool,
    #[sea_orm(iden = "class_1")]
    Class1,
    #[sea_orm(iden = "class_2")]
    Class2,
    #[sea_orm(iden = "class_3")]
    Class3,
    #[sea_orm(iden = "class_4")]
    Class4,
    #[sea_orm(iden = "class_5")]
    Class5,
    #[sea_orm(iden = "university")]
    University,
}
