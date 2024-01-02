use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::application::dto::response::book::MinBookResp;
use crate::application::entities::author::Author;
use crate::application::entities::book::Book;


/// Информация об одном авторе.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct FullAuthorResp {
  /// Уникальный идентификатор.
  #[schema(example = "6d786a4c-7262-439d-bfa3-7d8e6327bfd1")]
  pub id: Uuid,

  /// Имя.
  #[schema(example = "Вася")]
  pub first_name: String,

  /// Фамилия.
  #[schema(example = "Васин")]
  pub last_name: String,

  /// Отчество.
  #[schema(example = "Васильевич")]
  pub middle_name: Option<String>,

  /// Книги, написанные автором.
  pub books: Vec<MinBookResp>,
}

impl FullAuthorResp {
  pub fn new(db_author: Author, db_books: Vec<Book>) -> Self {
    Self {
      id: db_author.id,
      first_name: db_author.first_name,
      last_name: db_author.last_name,
      middle_name: db_author.middle_name,
      books: db_books.into_iter().map(MinBookResp::new).collect(),
    }
  }
}


/// Минимальная информация об одном авторе.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MinAuthorResp {
  /// Уникальный идентификатор.
  #[schema(example = "6d786a4c-7262-439d-bfa3-7d8e6327bfd1")]
  pub id: Uuid,

  /// Имя.
  #[schema(example = "Вася")]
  pub first_name: String,

  /// Фамилия.
  #[schema(example = "Васин")]
  pub last_name: String,

  /// Отчество.
  #[schema(example = "Васильевич")]
  pub middle_name: Option<String>,
}

impl MinAuthorResp {
  pub fn new(db_author: Author) -> Self {
    Self {
      id: db_author.id,
      first_name: db_author.first_name,
      last_name: db_author.last_name,
      middle_name: db_author.middle_name,
    }
  }
}


/// Информация о нескольких авторах.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AuthorListResp(pub Vec<FullAuthorResp>);

impl AuthorListResp {
  pub fn new(db_authors: Vec<Author>, db_books: Vec<Vec<Book>>) -> Self {
    let mut res = vec![];
    for (db_author, db_books) in db_authors.into_iter().zip(db_books) {
      res.push(FullAuthorResp::new(db_author, db_books));
    }
    Self(res)
  }
}
