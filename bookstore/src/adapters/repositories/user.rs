use std::error::Error;
use log;

use sqlx;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::application::dto::request::user::UpdateSuspendedReq;
use crate::application::entities::user::User;


pub struct UserRepository {
  conn_pool: Pool<Postgres>,
}

impl UserRepository {
  pub fn new(conn_pool: Pool<Postgres>) -> Self {
    Self {
      conn_pool
    }
  }

  /// Fetch user from the database by ID.
  pub async fn get_by_id(&self, id: &Uuid) -> Result<Option<User>, Box<dyn Error>> {
    let text = "SELECT * FROM users WHERE id = $1 LIMIT 1";
    let query = sqlx::query_as::<_, User>(text).bind(id);

    match query.fetch_optional(&self.conn_pool).await {
      Ok(user) => Ok(user),
      Err(e) => {
        log::error!("Error fetching user by id: {}", e);
        Err(Box::new(e))
      }
    }
  }

  /// Fetch user from the database by nickname.
  pub async fn get_by_nickname(&self, nickname: &String) -> Result<Option<User>, Box<dyn Error>> {
    let text = "SELECT * FROM users WHERE nickname = $1 LIMIT 1";
    let query = sqlx::query_as::<_, User>(text).bind(nickname);

    match query.fetch_optional(&self.conn_pool).await {
      Ok(user) => Ok(user),
      Err(e) => {
        log::error!("Error fetching user by nickname: {}", e);
        Err(Box::new(e))
      }
    }
  }

  /// Fetch users from the database.
  pub async fn get_list(&self, page: u32, size: u32) -> Result<Vec<User>, Box<dyn Error>> {
    let text = "SELECT * FROM users OFFSET $1 LIMIT $2";
    let query = sqlx::query_as::<_, User>(text)
      .bind((page * size) as i64)
      .bind(size as i64);

    match query.fetch_all(&self.conn_pool).await {
      Ok(users) => Ok(users),
      Err(e) => {
        log::error!("Error fetching users: {}", e);
        Err(Box::new(e))
      }
    }
  }

  /// Save user into the database.
  pub async fn add_one(&self, user: User) -> Result<(), Box<dyn Error>> {
    let text = concat!(
      "INSERT INTO users\n",
      "  (id, first_name, last_name, middle_name, nickname, hashed_password, date_registered, role, suspended)\n",
      "VALUES\n",
      "  ($1, $2, $3, $4, $5, $6, $7, $8, $9)"
    );
    let query = sqlx::query(text)
      .bind(user.id)
      .bind(user.first_name)
      .bind(user.last_name)
      .bind(user.middle_name)
      .bind(user.nickname)
      .bind(user.hashed_password)
      .bind(user.date_registered)
      .bind(user.role)
      .bind(user.suspended);

    match query.execute(&self.conn_pool).await {
      Ok(_) => Ok(()),
      Err(e) => {
        log::error!("Error adding user: {}", e);
        Err(Box::new(e))
      }
    }
  }

  /// Update user's `suspended` column in the database by ID.
  pub async fn update_suspended(&self, id: &Uuid, data: UpdateSuspendedReq) -> Result<Option<User>, Box<dyn Error>> {
    let update_text = "UPDATE users SET suspended = $1 WHERE id = $2";
    let update_query = sqlx::query(update_text)
      .bind(data.suspended)
      .bind(id);

    let fetch_text = "SELECT * FROM users WHERE id = $1 LIMIT 1";
    let fetch_query = sqlx::query_as::<_, User>(fetch_text)
      .bind(id);

    match update_query.execute(&self.conn_pool).await {
      Ok(_) => {},
      Err(e) => {
        log::error!("Error updating user: {}", e);
        return Err(Box::new(e))
      }
    };

    match fetch_query.fetch_optional(&self.conn_pool).await {
      Ok(user) => Ok(user),
      Err(e) => {
        log::error!("Error fetching user after update: {}", e);
        Err(Box::new(e))
      }
    }
  }
}