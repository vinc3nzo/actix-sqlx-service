use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;


/// Запрос на получение информации о нескольких книгах.
#[derive(Debug, Serialize, Deserialize)]
pub struct GetBookListReq {
  pub page: u32,
  pub size: u32,
}

/// Запрос на добавление книги.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AddBookReq {
  /// Название.
  #[schema(example = "Книга", min_length = 1)]
  pub title: String,

  /// Идентификатор автора книги.
  #[schema(example = "6d786a4c-7262-439d-bfa3-7d8e6327bfd1")]
  pub author_id: Option<Uuid>,
}
