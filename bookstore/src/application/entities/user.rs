use serde::{Deserialize, Serialize};
use chrono::{DateTime, Local};
use uuid::Uuid;
use utoipa::ToSchema;
use sqlx::{FromRow, Type};

use crate::application::dto::request::user::RegisterReq;


#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, ToSchema, Type)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRole {
  /// Regular user.
  User,

  /// Administrator.
  Admin,
}

// FromRow macro is a convenience trait for a row-to-object conversion by SQLx.
#[derive(Debug, Clone, FromRow)]
pub struct User {
  pub id: Uuid,
  pub first_name: String,
  pub last_name: String,
  pub middle_name: Option<String>,
  pub nickname: String,
  pub hashed_password: String,
  pub date_registered: DateTime<Local>,
  pub role: UserRole,
  pub suspended: bool,
}

impl User {
  pub fn new(value: RegisterReq) -> Self {
    Self {
      id: Uuid::new_v4(),
      first_name: value.first_name,
      last_name: value.last_name,
      middle_name: value.middle_name,
      nickname: value.nickname,
      hashed_password: "".to_string(),
      date_registered: Local::now(),
      role: UserRole::User,
      suspended: false,
    }
  }
}