use std::error::Error;
use uuid::Uuid;
use sqlx::{Pool, Postgres};

use crate::application::entities::book::Book;


pub struct BookRepository {
  conn_pool: Pool<Postgres>,
}

impl BookRepository {
  pub fn new(conn_pool: Pool<Postgres>) -> Self {
    Self {
      conn_pool,
    }
  }

  /// Fetch book from the database by ID.
  pub async fn get_by_id(&self, id: &Uuid) -> Result<Option<Book>, Box<dyn Error>> {
    let text = "SELECT * FROM books WHERE id = $1 LIMIT 1";
    let query = sqlx::query_as::<_, Book>(text).bind(id);

    match query.fetch_optional(&self.conn_pool).await {
      Ok(book) => Ok(book),
      Err(e) => {
        log::error!("Error fetching book by id: {}", e);
        Err(Box::new(e))
      }
    }
  }

  /// Fetch books from the database by `author_id`.
  pub async fn get_by_author_id(&self, author_id: &Uuid) -> Result<Vec<Book>, Box<dyn Error>> {
    let text = "SELECT * FROM books WHERE author_id = $1";
    let query = sqlx::query_as::<_, Book>(text).bind(author_id);

    match query.fetch_all(&self.conn_pool).await {
      Ok(book) => Ok(book),
      Err(e) => {
        log::error!("Error fetching books by author_id: {}", e);
        Err(Box::new(e))
      }
    }
  }

  /// Fetch books from the database.
  pub async fn get_list(&self, page: u32, size: u32) -> Result<Vec<Book>, Box<dyn Error>> {
    let text = "SELECT * FROM books OFFSET $1 LIMIT $2";
    let query = sqlx::query_as::<_, Book>(text)
      .bind((page * size) as i64)
      .bind(size as i64);

    match query.fetch_all(&self.conn_pool).await {
      Ok(books) => Ok(books),
      Err(e) => {
        log::error!("Error fetching books: {}", e);
        Err(Box::new(e))
      }
    }
  }

  /// Save book into the database.
  pub async fn add_one(&self, book: Book) -> Result<(), Box<dyn Error>> {
    let text = concat!(
      "INSERT INTO books\n",
      "  (id, title, author_id)\n",
      "VALUES\n",
      "  ($1, $2, $3)"
    );
    let query = sqlx::query(text)
      .bind(book.id)
      .bind(book.title)
      .bind(book.author_id);

    match query.execute(&self.conn_pool).await {
      Ok(_) => Ok(()),
      Err(e) => {
        log::error!("Error adding book: {}", e);
        Err(Box::new(e))
      }
    }
  }

  /// Delete book from the database by ID.
  pub async fn delete_one(&self, id: &Uuid) -> Result<(), Box<dyn Error>> {
    let text = "DELETE FROM books WHERE id = $1";
    let query = sqlx::query(text).bind(id);

    match query.execute(&self.conn_pool).await {
      Ok(_) => Ok(()),
      Err(e) => {
        log::error!("Error deleting book: {}", e);
        Err(Box::new(e))
      }
    }
  }
}