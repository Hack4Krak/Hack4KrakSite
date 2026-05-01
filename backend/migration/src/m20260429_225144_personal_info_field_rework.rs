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
            .alter_table(
                Table::alter()
                    .table(UserPersonalInfo::Table)
                    .drop_column(UserPersonalInfo::BirthYear)
                    .drop_column(UserPersonalInfo::Organization)
                    .drop_column(UserPersonalInfo::IsVegetarian)
                    .add_column(
                        ColumnDef::new(UserPersonalInfo::CtfExperience)
                            .custom(CtfExperience::Enum)
                            .not_null()
                            .extra("DEFAULT 'never'"),
                    )
                    .add_column(
                        ColumnDef::new(UserPersonalInfo::SchoolGrade)
                            .custom(SchoolGrade::Enum)
                            .not_null()
                            .extra("DEFAULT 'not_studying'"),
                    )
                    .add_column(
                        ColumnDef::new(UserPersonalInfo::CollabInterest)
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
                    .table(UserPersonalInfo::Table)
                    .rename_column(
                        UserPersonalInfo::ReferralSource,
                        UserPersonalInfo::ReferralSources,
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
                UPDATE user_personal_info
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
                    .table(UserPersonalInfo::Table)
                    .rename_column(
                        UserPersonalInfo::ReferralSources,
                        UserPersonalInfo::ReferralSource,
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(UserPersonalInfo::Table)
                    .drop_column(UserPersonalInfo::CtfExperience)
                    .drop_column(UserPersonalInfo::SchoolGrade)
                    .drop_column(UserPersonalInfo::CollabInterest)
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
enum UserPersonalInfo {
    Table,
    BirthYear,
    Organization,
    IsVegetarian,
    CtfExperience,
    SchoolGrade,
    CollabInterest,
    ReferralSource,
    ReferralSources,
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
