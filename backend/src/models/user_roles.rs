use crate::entities::sea_orm_active_enums::UserRoles;
use std::borrow::Cow;
use utoipa::__dev::ComposeSchema;
use utoipa::ToSchema;
use utoipa::openapi::{RefOr, Schema, schema};

impl Copy for UserRoles {}

impl ComposeSchema for UserRoles {
    fn compose(_generics: Vec<RefOr<Schema>>) -> RefOr<Schema> {
        schema::Object::builder()
            .schema_type(schema::SchemaType::new(schema::Type::String))
            .enum_values::<[&str; 3usize], &str>(Some(["Default", "Admin", "Owner"]))
            .into()
    }
}

impl ToSchema for UserRoles {
    fn name() -> Cow<'static, str> {
        Cow::Borrowed("UserRoles")
    }

    fn schemas(schemas: &mut Vec<(String, RefOr<Schema>)>) {
        schemas.extend([]);
    }
}
