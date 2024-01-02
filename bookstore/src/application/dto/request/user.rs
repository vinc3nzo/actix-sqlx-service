use serde::{Deserialize, Serialize};
use utoipa::ToSchema;


/// Запрос на регистрацию пользователя.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RegisterReq {
  /// Имя.
  #[schema(example = "Вася", min_length = 1)]
  pub first_name: String,

  /// Фамилия.
  #[schema(example = "Васин", min_length = 1)]
  pub last_name: String,

  /// Отчество.
  #[schema(example = "Васильевич", min_length = 1)]
  pub middle_name: Option<String>,

  /// Псведоним.
  ///
  /// Непустая строка, состоящая как минимум из трех
  /// английских букв, цифр и символов `.`, `-`, `_`.
  #[schema(example = "Aboba_x69", min_length = 3)]
  pub nickname: String,

  /// Пароль.
  #[schema(example = "password", min_length = 1)]
  pub password: String,
}

/// Запрос на авторизацию пользователя.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LoginReq {
  /// Псведоним.
  #[schema(example = "Aboba_x69")]
  pub nickname: String,

  /// Пароль.
  #[schema(example = "password")]
  pub password: String,
}

/// Запрос на получение информации о нескольких пользователях.
#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserListReq {
  pub page: u32,
  pub size: u32,
}

/// Запрос на обновление статуса действия аккаунта пользователя.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateSuspendedReq {
  pub suspended: bool,
}