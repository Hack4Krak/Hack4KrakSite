use crate::entities::sea_orm_active_enums::CtfExperience;
use std::borrow::Cow;
use utoipa::__dev::ComposeSchema;
use utoipa::ToSchema;
use utoipa::openapi::{RefOr, Schema, schema};

#[allow(clippy::derivable_impls)]
impl Default for CtfExperience {
    fn default() -> Self {
        CtfExperience::Never
    }
}

impl ComposeSchema for CtfExperience {
    fn compose(_generics: Vec<RefOr<Schema>>) -> RefOr<Schema> {
        schema::Object::builder()
            .schema_type(schema::SchemaType::new(schema::Type::String))
            .enum_values::<[&str; 5usize], &str>(Some([
                "Never",
                "Beginner",
                "Intermediate",
                "Advanced",
                "Expert",
            ]))
            .into()
    }
}

impl ToSchema for CtfExperience {
    fn name() -> Cow<'static, str> {
        Cow::Borrowed("CtfExperience")
    }

    fn schemas(schemas: &mut Vec<(String, RefOr<Schema>)>) {
        schemas.extend([]);
    }
}
