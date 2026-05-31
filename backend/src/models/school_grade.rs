use crate::entities::sea_orm_active_enums::SchoolGrade;
use std::borrow::Cow;
use utoipa::__dev::ComposeSchema;
use utoipa::ToSchema;
use utoipa::openapi::{RefOr, Schema, schema};

#[allow(clippy::derivable_impls)]
impl Default for SchoolGrade {
    fn default() -> Self {
        SchoolGrade::NotStudying
    }
}

impl ComposeSchema for SchoolGrade {
    fn compose(_generics: Vec<RefOr<Schema>>) -> RefOr<Schema> {
        schema::Object::builder()
            .schema_type(schema::SchemaType::new(schema::Type::String))
            .enum_values::<[&str; 8usize], &str>(Some([
                "NotStudying",
                "PrimarySchool",
                "Class1",
                "Class2",
                "Class3",
                "Class4",
                "Class5",
                "University",
            ]))
            .into()
    }
}

impl ToSchema for SchoolGrade {
    fn name() -> Cow<'static, str> {
        Cow::Borrowed("SchoolGrade")
    }

    fn schemas(schemas: &mut Vec<(String, RefOr<Schema>)>) {
        schemas.extend([]);
    }
}
