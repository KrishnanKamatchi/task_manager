use serde::{Deserialize, Serialize};
use sqlx::FromRow;
//use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub description: String,
    pub completed: bool,
}
