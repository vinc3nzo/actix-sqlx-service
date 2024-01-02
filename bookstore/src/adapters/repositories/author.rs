use std::error::Error;
use uuid::Uuid;
use sqlx::{Pool, Postgres};

use crate::application::entities::author::Author;


pub struct AuthorRepository {
  conn_pool: Pool<Postgres>,
}

impl AuthorRepository {
  pub fn new(conn_pool: Pool<Postgres>) -> Self {
    Self {
      conn_pool,
    }
  }

  /// Fetch author from the database by ID.
  pub async fn get_by_id(&self, id: &Uuid) -> Result<Option<Author>, Box<dyn Error>> {
    let text = "SELECT * FROM authors WHERE id = $1 LIMIT 1";
    let query = sqlx::query_as::<_, Author>(text).bind(id);

    match query.fetch_optional(&self.conn_pool).await {
      Ok(author) => Ok(author),
      Err(e) => {
        log::error!("Error fetching author by id: {}", e);
        Err(Box::new(e))
      }
    }
  }

  /// Fetch authors from the database.
  pub async fn get_list(&self, page: u32, size: u32) -> Result<Vec<Author>, Box<dyn Error>> {
    let text = "SELECT * FROM authors OFFSET $1 LIMIT $2";
    let query = sqlx::query_as::<_, Author>(text)
      .bind((page * size) as i64)
      .bind(size as i64);

    match query.fetch_all(&self.conn_pool).await {
      Ok(authors) => Ok(authors),
      Err(e) => {
        log::error!("Error fetching authors: {}", e);
        Err(Box::new(e))
      }
    }
  }

  /// Save author into the database.
  pub async fn add_one(&self, author: Author) -> Result<(), Box<dyn Error>> {
    let text = concat!(
    "INSERT INTO authors\n",
    "  (id, first_name, last_name, middle_name)\n",
    "VALUES\n",
    "  ($1, $2, $3, $4)"
    );
    let query = sqlx::query(text)
      .bind(author.id)
      .bind(author.first_name)
      .bind(author.last_name)
      .bind(author.middle_name);

    match query.execute(&self.conn_pool).await {
      Ok(_) => Ok(()),
      Err(e) => {
        log::error!("Error adding author: {}", e);
        Err(Box::new(e))
      }
    }
  }

  /// Delete author from the database by ID.
  pub async fn delete_one(&self, id: &Uuid) -> Result<(), Box<dyn Error>> {
    let text = "DELETE FROM authors WHERE id = $1";
    let query = sqlx::query(text).bind(id);

    match query.execute(&self.conn_pool).await {
      Ok(_) => Ok(()),
      Err(e) => {
        log::error!("Error deleting author: {}", e);
        Err(Box::new(e))
      }
    }
  }
}