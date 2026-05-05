use crate::entities::sea_orm_active_enums::FoodPreference;
use std::borrow::Cow;
use utoipa::__dev::ComposeSchema;
use utoipa::ToSchema;
use utoipa::openapi::{RefOr, Schema, schema};

impl Default for FoodPreference {
    fn default() -> Self {
        FoodPreference::Standard
    }
}

impl ComposeSchema for FoodPreference {
    fn compose(_generics: Vec<RefOr<Schema>>) -> RefOr<Schema> {
        schema::Object::builder()
            .schema_type(schema::SchemaType::new(schema::Type::String))
            .enum_values::<[&str; 2usize], &str>(Some(["Standard", "Vegetarian"]))
            .into()
    }
}

impl ToSchema for FoodPreference {
    fn name() -> Cow<'static, str> {
        Cow::Borrowed("FoodPreference")
    }

    fn schemas(schemas: &mut Vec<(String, RefOr<Schema>)>) {
        schemas.extend([]);
    }
}
