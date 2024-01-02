use sqlx::FromRow;
use uuid::Uuid;

use crate::application::dto::request::author::AddAuthorReq;


// FromRow macro is a convenience trait for a row-to-object conversion by SQLx.
#[derive(Debug, Clone, FromRow)]
pub struct Author {
  pub id: Uuid,
  pub first_name: String,
  pub last_name: String,
  pub middle_name: Option<String>,
}


impl Author {
  pub fn new(value: AddAuthorReq) -> Self {
    Self {
      id: Uuid::new_v4(),
      first_name: value.first_name,
      last_name: value.last_name,
      middle_name: value.middle_name,
    }
  }
}