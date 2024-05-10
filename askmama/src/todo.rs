pub struct Todo {
    pub id: uuid::Uuid,
    pub title: String,
    pub completed: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
