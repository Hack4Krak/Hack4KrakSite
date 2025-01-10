use sea_orm::DatabaseConnection;

pub struct AppState {
    #[allow(unused)]
    pub database: DatabaseConnection,
}
