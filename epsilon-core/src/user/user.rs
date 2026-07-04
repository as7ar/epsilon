use uuid::Uuid;

pub struct User {
    pub id: Uuid,
    pub nickname: String,
    pub created_at: i64
}