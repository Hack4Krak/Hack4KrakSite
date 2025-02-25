use crate::entities::sea_orm_active_enums::TeamStatus;

#[allow(clippy::derivable_impls)]
impl Default for TeamStatus {
    fn default() -> Self {
        TeamStatus::Absent
    }
}
