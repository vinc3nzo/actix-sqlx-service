use std::error::Error;
use std::sync::Arc;
use uuid::Uuid;

use crate::adapters::repositories::book::BookRepository;
use crate::adapters::repositories::author::AuthorRepository;
use crate::application::dto::request::author::{AddAuthorReq, GetAuthorListReq};
use crate::application::dto::response::author::FullAuthorResp;
use crate::application::entities::author::Author;


pub struct AuthorService
{
  author_repo: Arc<AuthorRepository>,
  book_repo: Arc<BookRepository>,
}

pub enum AuthorFetchResult {
  Ok(FullAuthorResp),
  NotFound,
  UnexpectedError(Box<dyn Error>),
}

pub enum AuthorAddResult {
  Created,
  UnexpectedError(Box<dyn Error>),
}

pub enum AuthorListFetchResult {
  Ok(Vec<FullAuthorResp>),
  UnexpectedError(Box<dyn Error>),
}

pub enum AuthorDeleteResult {
  Ok,
  UnexpectedError(Box<dyn Error>),
}

impl AuthorService
{
  pub fn new(author_repo: Arc<AuthorRepository>, book_repo: Arc<BookRepository>) -> Self {
    Self {
      author_repo,
      book_repo,
    }
  }

  pub async fn get_by_id(&self, id: &Uuid) -> AuthorFetchResult {
    match self.author_repo.get_by_id(id).await {
      Ok(author) => match author {
        Some(author) => {
          match self.book_repo.get_by_author_id(&author.id).await {
            Ok(books) => AuthorFetchResult::Ok(FullAuthorResp::new(author, books)),
            Err(e) => AuthorFetchResult::UnexpectedError(e),
          }
        },
        None => AuthorFetchResult::NotFound,
      },
      Err(e) => AuthorFetchResult::UnexpectedError(e),
    }
  }

  pub async fn add_one(&self, data: AddAuthorReq) -> AuthorAddResult {
    match self.author_repo.add_one(Author::new(data)).await {
      Ok(_) => AuthorAddResult::Created,
      Err(e) => AuthorAddResult::UnexpectedError(e),
    }
  }

  pub async fn get_list(&self, params: GetAuthorListReq) -> AuthorListFetchResult {
    match self.author_repo.get_list(params.page, params.size).await {
      Ok(authors) => {
        let mut books = vec![];
        for author in authors.iter() {
          match self.book_repo.get_by_author_id(&author.id).await {
            Ok(author_books) => books.push(author_books),
            Err(e) => return AuthorListFetchResult::UnexpectedError(e),
          }
        }
        let res = authors.into_iter().zip(books).map(|(a, bs)| FullAuthorResp::new(a, bs)).collect();
        AuthorListFetchResult::Ok(res)
      }
      Err(e) => AuthorListFetchResult::UnexpectedError(e),
    }
  }

  pub async fn delete_one(&self, id: &Uuid) -> AuthorDeleteResult {
    match self.author_repo.delete_one(id).await {
      Ok(_) => AuthorDeleteResult::Ok,
      Err(e) => AuthorDeleteResult::UnexpectedError(e),
    }
  }
}