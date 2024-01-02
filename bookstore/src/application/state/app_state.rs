use std::sync::Arc;

use crate::application::services::auth::AuthService;
use crate::application::services::book::BookService;
use crate::application::services::user::UserService;
use crate::application::services::author::AuthorService;


pub struct AppState
{
  pub user_service: Arc<UserService>,
  pub auth_service: Arc<AuthService>,
  pub book_service: Arc<BookService>,
  pub author_service: Arc<AuthorService>,
}