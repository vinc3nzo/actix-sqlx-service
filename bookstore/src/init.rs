use std::sync::Arc;
use actix_web::web;
use sqlx;

use bookstore::adapters::repositories::author::AuthorRepository;
use bookstore::adapters::repositories::book::BookRepository;

// use bookstore::add_admin_user;
use bookstore::application::services::auth::AuthService;
use bookstore::application::services::user::UserService;
use bookstore::application::state::app_state::AppState;

use bookstore::adapters::repositories::user::UserRepository;
use bookstore::application::services::author::AuthorService;
use bookstore::application::services::book::BookService;

use crate::db_conn::get_db_url;


pub struct InitData {
  pub host: String,
  pub port: u16,
  pub app_state: web::Data<AppState>,
}

pub async fn init() -> InitData {
  // Environment variables
  std::env::var("APP_SECRET").expect("please set the `APP_SECRET` environment variable");
  let host = std::env::var("APP_HOST").unwrap_or("0.0.0.0".to_string());
  let port = std::env::var("APP_PORT").unwrap_or("3000".to_string())
    .parse::<u16>()
    .unwrap();

  let _admin_username = std::env::var("APP_ADMIN_USERNAME").unwrap_or("admin".to_string());
  let _admin_password = std::env::var("APP_ADMIN_PASSWORD").unwrap_or("1234".to_string());

  // Database connection
  let db_url = get_db_url();
  let conn_pool = sqlx::postgres::PgPool::connect(&db_url).await.unwrap();

  // Database migrations
  sqlx::migrate!("./migrations").run(&conn_pool).await.unwrap();

  // Repositories
  let user_repository = Arc::new(
    UserRepository::new(conn_pool.clone())
  );
  let book_repository = Arc::new(
    BookRepository::new(conn_pool.clone())
  );
  let author_repository = Arc::new(
    AuthorRepository::new(conn_pool)
  );

  // Services
  let user_service = Arc::new(UserService::new(user_repository.clone()));
  let auth_service = Arc::new(AuthService::new(user_repository));
  let book_service = Arc::new(BookService::new(book_repository.clone(), author_repository.clone()));
  let author_service = Arc::new(AuthorService::new(author_repository, book_repository));

  // add_admin_user(user_service.clone(), admin_username, admin_password).await;

  let app_state = web::Data::new(
    AppState {
      user_service,
      auth_service,
      book_service,
      author_service,
    }
  );

  InitData {
    host,
    port,
    app_state,
  }
}