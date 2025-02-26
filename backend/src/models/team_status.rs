use crate::entities::sea_orm_active_enums::TeamStatus;
use std::borrow::Cow;
use utoipa::__dev::ComposeSchema;
use utoipa::ToSchema;
use utoipa::openapi::{RefOr, Schema, schema};

#[allow(clippy::derivable_impls)]
impl Default for TeamStatus {
    fn default() -> Self {
        TeamStatus::Absent
    }
}

impl ComposeSchema for TeamStatus {
    fn compose(_generics: Vec<RefOr<Schema>>) -> RefOr<Schema> {
        schema::Object::builder()
            .schema_type(schema::SchemaType::new(schema::Type::String))
            .enum_values::<[&str; 2usize], &str>(Some(["Confirmed", "Absent"]))
            .into()
    }
}

impl ToSchema for TeamStatus {
    fn name() -> Cow<'static, str> {
        Cow::Borrowed("TeamStatus")
    }

    fn schemas(schemas: &mut Vec<(String, RefOr<Schema>)>) {
        schemas.extend([]);
    }
}
