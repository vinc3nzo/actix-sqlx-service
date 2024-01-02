use std::error::Error;
use std::sync::Arc;
use uuid::Uuid;

use crate::adapters::repositories::author::AuthorRepository;
use crate::adapters::repositories::book::BookRepository;
use crate::application::dto::request::book::{AddBookReq, GetBookListReq};
use crate::application::dto::response::book::FullBookResp;
use crate::application::entities::book::Book;


pub struct BookService
{
  book_repo: Arc<BookRepository>,
  author_repo: Arc<AuthorRepository>,
}

pub enum BookFetchResult {
  Ok(FullBookResp),
  NotFound,
  UnexpectedError(Box<dyn Error>),
}

pub enum BookAddResult {
  Created,
  AuthorNotFound,
  UnexpectedError(Box<dyn Error>),
}

pub enum BookListFetchResult {
  Ok(Vec<FullBookResp>),
  UnexpectedError(Box<dyn Error>),
}

pub enum BookDeleteResult {
  Ok,
  UnexpectedError(Box<dyn Error>),
}

impl BookService
{
  pub fn new(book_repo: Arc<BookRepository>, author_repo: Arc<AuthorRepository>) -> Self {
    Self {
      book_repo,
      author_repo,
    }
  }

  pub async fn get_by_id(&self, id: &Uuid) -> BookFetchResult {
    match self.book_repo.get_by_id(id).await {
      Ok(book) => match book {
        Some(book) => match book.author_id {
          Some(author_id) => match self.author_repo.get_by_id(&author_id).await {
            Ok(author) => BookFetchResult::Ok(FullBookResp::new(book, author)),
            Err(e) => BookFetchResult::UnexpectedError(e),
          },
          None => BookFetchResult::Ok(FullBookResp::new(book, None)),
        },
        None => BookFetchResult::NotFound,
      },
      Err(e) => BookFetchResult::UnexpectedError(e),
    }
  }

  pub async fn add_one(&self, data: AddBookReq) -> BookAddResult {
    if let Some(author_id) = data.author_id {
      match self.author_repo.get_by_id(&author_id).await {
        Ok(author) => {
          if let None = author {
            return BookAddResult::AuthorNotFound
          }
        },
        Err(e) => return BookAddResult::UnexpectedError(e),
      }
    }

    match self.book_repo.add_one(Book::new(data)).await {
      Ok(_) => BookAddResult::Created,
      Err(e) => BookAddResult::UnexpectedError(e),
    }
  }

  pub async fn get_list(&self, params: GetBookListReq) -> BookListFetchResult {
    match self.book_repo.get_list(params.page, params.size).await {
      Ok(books) => {
        let mut authors = vec![];
        for book in books.iter() {
          match book.author_id {
            Some(author_id) => {
              match self.author_repo.get_by_id(&author_id).await {
                Ok(author) => authors.push(author),
                Err(e) => return BookListFetchResult::UnexpectedError(e),
              }
            },
            None => authors.push(None),
          }
        }
        let res = books.into_iter().zip(authors).map(|(b, a)| FullBookResp::new(b, a)).collect();
        BookListFetchResult::Ok(res)
      },
      Err(e) => BookListFetchResult::UnexpectedError(e),
    }
  }

  pub async fn delete_one(&self, id: &Uuid) -> BookDeleteResult {
    match self.book_repo.delete_one(id).await {
      Ok(_) => BookDeleteResult::Ok,
      Err(e) => BookDeleteResult::UnexpectedError(e),
    }
  }
}