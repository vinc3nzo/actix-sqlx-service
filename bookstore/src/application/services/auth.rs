use std::fmt::{Display, Formatter};
use std::sync::Arc;
use std::error::Error;
use derive_more::Error;
use bcrypt;
use regex::Regex;

use crate::application::dto::request::user::{LoginReq, RegisterReq};
use crate::application::entities::user::User;
use crate::adapters::middleware::jwt::JwtClaims;
use crate::adapters::repositories::user::UserRepository;


#[derive(Debug, Clone, Error)]
pub enum RegistrationError {
  AlreadyExists,
  UnexpectedError,
  BadRequest,
}

impl Display for RegistrationError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      RegistrationError::AlreadyExists => write!(f, "User with this nickname already exists"),
      RegistrationError::UnexpectedError => write!(f, "Internal error"),
      RegistrationError::BadRequest => write!(f, "Bad request format"),
    }
  }
}

#[derive(Debug, Clone, Error)]
pub enum LoginError {
  UserNotFound,
  UnexpectedError,
}

impl Display for LoginError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      LoginError::UserNotFound => write!(f, "User not found"),
      LoginError::UnexpectedError => write!(f, "Internal error"),
    }
  }
}


pub struct AuthService
{
  user_repo: Arc<UserRepository>,
}

impl AuthService
{
  pub fn new(user_repo: Arc<UserRepository>) -> Self {
    Self {
      user_repo
    }
  }

  fn check_nickname(nickname: &String) -> bool {
    let re = Regex::new(r"^[a-zA-Z0-9.\-_]+$").unwrap();
    re.is_match(nickname)
  }

  fn check_password(password: &String) -> bool {
    let re = Regex::new(r"^.+$").unwrap();
    re.is_match(password)
  }

  fn check_name(name: &String) -> bool {
    let re = Regex::new(r"^[a-zA-Zа-яА-Я]+$").unwrap();
    re.is_match(name)
  }

  pub async fn register(&self, data: RegisterReq) -> Result<String, RegistrationError>
  {
    if !Self::check_nickname(&data.nickname)
      || !Self::check_password(&data.password)
      || !Self::check_name(&data.first_name)
      || !Self::check_name(&data.last_name)
    {
      return Err(RegistrationError::BadRequest);
    }

    match self.user_repo.get_by_nickname(&data.nickname).await {
      Ok(user) => match user {
        Some(_) => return Err(RegistrationError::AlreadyExists),
        None => {}
      },
      Err(_) => return Err(RegistrationError::UnexpectedError),
    };

    let hashed_password = match bcrypt::hash(data.password.clone(), 5) {
      Ok(hashed_password) => hashed_password,
      Err(_) => return Err(RegistrationError::UnexpectedError),
    };

    let mut new_user = User::new(data);
    new_user.hashed_password = hashed_password;

    match self.user_repo.add_one(new_user.clone()).await {
      Ok(_) => {
        let claims = JwtClaims::new(new_user.id, new_user.role);
        Ok(claims.to_token())
      },
      Err(_) => {
        Err(RegistrationError::UnexpectedError)
      }
    }
  }

  pub async fn login(&self, data: LoginReq) -> Result<Option<String>, Box<dyn Error>> {
    let user = match self.user_repo.get_by_nickname(&data.nickname).await {
      Ok(user) => match user {
        Some(user) => user,
        None => return Ok(None),
      },
      Err(e) => return Err(e),
    };

    match bcrypt::verify(&data.password, &user.hashed_password) {
      Err(e) => Err(Box::new(e)),
      Ok(b) => {
        if b {
          let claims = JwtClaims::new(user.id, user.role);
          Ok(Some(claims.to_token()))
        } else {
          Ok(None)
        }
      }
    }
  }
}