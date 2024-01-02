use sqlx::FromRow;
use uuid::Uuid;
use crate::application::dto::request::book::AddBookReq;


// FromRow macro is a convenience trait for a row-to-object conversion by SQLx.
#[derive(Debug, Clone, FromRow)]
pub struct Book {
  pub id: Uuid,
  pub title: String,
  pub author_id: Option<Uuid>,
}

impl Book {
  pub fn new(value: AddBookReq) -> Self {
    Self {
      id: Uuid::new_v4(),
      title: value.title,
      author_id: value.author_id,
    }
  }
}