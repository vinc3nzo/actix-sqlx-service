use serde::{Deserialize, Serialize};
use utoipa::ToSchema;


/// Запрос на получение информации о нескольких авторах.
#[derive(Debug, Serialize, Deserialize)]
pub struct GetAuthorListReq {
  pub page: u32,
  pub size: u32,
}


/// Запрос на добавление автора.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AddAuthorReq {
  /// Имя.
  #[schema(example = "Вася")]
  pub first_name: String,

  /// Фамилия.
  #[schema(example = "Васин")]
  pub last_name: String,

  /// Отчество.
  #[schema(example = "Васильевич")]
  pub middle_name: Option<String>,
}
