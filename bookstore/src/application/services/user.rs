use std::error::Error;
use std::sync::Arc;
use uuid::Uuid;

use crate::adapters::repositories::user::UserRepository;
use crate::application::dto::request::user::{GetUserListReq, RegisterReq, UpdateSuspendedReq};
use crate::application::dto::response::user::{FullUserResp, UserListResp};
use crate::application::entities::user::User;


pub struct UserService
{
  user_repo: Arc<UserRepository>,
}

pub enum UserFetchResult {
  Ok(FullUserResp),
  NotFound,
  UnexpectedError(Box<dyn Error>),
}

pub enum UserListFetchResult {
  Ok(UserListResp),
  UnexpectedError(Box<dyn Error>),
}

pub enum UserAddResult {
  Created,
  UnexpectedError(Box<dyn Error>),
}

pub enum UserUpdateSuspendedResult {
  Ok(FullUserResp),
  NotFound,
  UnexpectedError(Box<dyn Error>),
}

impl UserService
{
  pub fn new(user_repo: Arc<UserRepository>) -> Self {
    Self {
      user_repo
    }
  }

  pub async fn get_by_id(&self, id: &Uuid) -> UserFetchResult {
    match self.user_repo.get_by_id(id).await {
      Ok(user) => match user {
        Some(user) => UserFetchResult::Ok(FullUserResp::new(user)),
        None => UserFetchResult::NotFound,
      },
      Err(e) => UserFetchResult::UnexpectedError(e),
    }
  }

  pub async fn get_by_nickname(&self, nickname: &String) -> UserFetchResult {
    match self.user_repo.get_by_nickname(nickname).await {
      Ok(user) => match user {
        Some(user) => UserFetchResult::Ok(FullUserResp::new(user)),
        None => UserFetchResult::NotFound,
      },
      Err(e) => UserFetchResult::UnexpectedError(e),
    }
  }

  pub async fn add_one(&self, user: RegisterReq) -> UserAddResult {
    match self.user_repo.add_one(User::new(user)).await {
      Ok(_) => UserAddResult::Created,
      Err(e) => UserAddResult::UnexpectedError(e),
    }
  }

  pub async fn get_list(&self, params: GetUserListReq) -> UserListFetchResult {
    match self.user_repo.get_list(params.page, params.size).await {
      Ok(users) => {
        let res = users.into_iter().map(|user| FullUserResp::new(user)).collect();
        UserListFetchResult::Ok(UserListResp(res))
      }
      Err(e) => UserListFetchResult::UnexpectedError(e),
    }
  }

  pub async fn update_suspended(&self, id: &Uuid, data: UpdateSuspendedReq) -> UserUpdateSuspendedResult {
    match self.user_repo.update_suspended(id, data).await {
      Ok(user) => match user {
        Some(user) => UserUpdateSuspendedResult::Ok(FullUserResp::new(user)),
        None => UserUpdateSuspendedResult::NotFound,
      }
      Err(e) => UserUpdateSuspendedResult::UnexpectedError(e),
    }
  }
}