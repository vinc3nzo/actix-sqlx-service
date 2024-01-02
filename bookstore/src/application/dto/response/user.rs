use chrono::{DateTime, Local};
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::application::entities::user::{User, UserRole};


/// Информация об одном пользователе.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct FullUserResp {
  /// Уникальный идентификатор.
  #[schema(example = "6d786a4c-7262-439d-bfa3-7d8e6327bfd1")]
  pub id: Uuid,

  /// Имя.
  #[schema(example = "Вася")]
  pub first_name: String,

  /// Фамилия.
  #[schema(example = "Васин")]
  pub last_name: String,

  /// Отчество.
  #[schema(example = "Васильевич")]
  pub middle_name: Option<String>,

  /// Псевдоним.
  #[schema(example = "username")]
  pub nickname: String,

  /// Время регистрации.
  #[schema(example = "2024-01-01T10:00:00+0400")]
  pub date_registered: DateTime<Local>,

  /// Роль пользователя.
  #[schema(example = UserRole::User)]
  pub role: UserRole,

  /// Запись с информацией о приостановке аккаунта.
  pub suspended: bool,
}

impl FullUserResp {
  pub fn new(value: User) -> Self {
    Self {
      id: value.id,
      first_name: value.first_name,
      last_name: value.last_name,
      middle_name: value.middle_name,
      nickname: value.nickname,
      date_registered: value.date_registered,
      role: value.role,
      suspended: value.suspended,
    }
  }
}


/// Информация о нескольких пользователях.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserListResp(pub Vec<FullUserResp>);

impl UserListResp {
  pub fn new(value: Vec<User>) -> Self {
    Self(value.into_iter().map(FullUserResp::new).collect())
  }
}


/// Ответ с токеном для авторизации.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TokenResp {
  /// Токен.
  #[schema(example = "jwt")]
  pub token: String,
}
