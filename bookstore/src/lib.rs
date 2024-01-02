#[macro_use]
extern crate actix_web;

use std::sync::Arc;

use crate::application::services::user::UserService;

pub mod application;
pub mod adapters;

pub async fn add_admin_user(_user_service: Arc<UserService>, _nickname: String, _password: String)
{
  todo!();
}