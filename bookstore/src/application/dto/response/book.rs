use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::application::dto::response::author::MinAuthorResp;
use crate::application::entities::author::Author;
use crate::application::entities::book::Book;


/// Информация об одной книге.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct FullBookResp {
  /// Уникальный идентификатор.
  #[schema(example = "6d786a4c-7262-439d-bfa3-7d8e6327bfd1")]
  pub id: Uuid,

  /// Название.
  #[schema(example = "Книга")]
  pub title: String,

  /// Идентификатор автора книги.
  #[schema(example = "6d786a4c-7262-439d-bfa3-7d8e6327bfd1")]
  pub author_id: Option<Uuid>,

  /// Информация об авторе книги.
  pub author: Option<MinAuthorResp>,
}

impl FullBookResp {
  pub fn new(db_book: Book, db_author: Option<Author>) -> Self {
    Self {
      id: db_book.id,
      title: db_book.title,
      author_id: db_book.author_id,
      author: db_author.map_or_else(
        || None,
        |a| Some(MinAuthorResp::new(a))
      ),
    }
  } 
}


/// Минимальная информация об одной книге.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MinBookResp {
  /// Уникальный идентификатор.
  #[schema(example = "6d786a4c-7262-439d-bfa3-7d8e6327bfd1")]
  pub id: Uuid,

  /// Название.
  #[schema(example = "Книга")]
  pub title: String,

  /// Идентификатор автора книги.
  #[schema(example = "6d786a4c-7262-439d-bfa3-7d8e6327bfd1")]
  pub author_id: Option<Uuid>,
}

impl MinBookResp {
  pub fn new(db_book: Book) -> Self {
    Self {
      id: db_book.id,
      title: db_book.title,
      author_id: db_book.author_id,
    }
  }
}


/// Информация о нескольких книгах.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct BookListResp(pub Vec<FullBookResp>);

impl BookListResp {
  pub fn new(db_books: Vec<Book>, db_authors: Vec<Option<Author>>) -> Self {
    let mut res = vec![];
    for (db_book, db_author) in db_books.into_iter().zip(db_authors) {
      res.push(FullBookResp::new(db_book, db_author));
    }
    Self(res)
  }
}

