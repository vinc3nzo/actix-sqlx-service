use actix_web::dev::Server;
use actix_web::{App, HttpServer, web};
use actix_web::middleware::Logger;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use bookstore::application::entities::user::UserRole;
use bookstore::application::state::app_state::AppState;
use bookstore::adapters::middleware::jwt::JwtAuth;
use bookstore::adapters::routes::{ping, user, auth, book, author};

use crate::api_docs::ApiDoc;

pub fn create_server(host: &str, port: u16, app_state: web::Data<AppState>) -> Server {
  let enable_docs: bool = std::env::var("APP_DOCS_ON")
    .unwrap_or("false".to_string())
    .parse()
    .expect("boolean expected");

  // this move-block is executed once per worker thread
  HttpServer::new(move || {
    let app_builder = App::new()
      .app_data(app_state.clone());

    let app_builder = if enable_docs {
      app_builder.service(
        SwaggerUi::new("/docs/{_:.*}")
          .url("/api-docs/openapi.json", ApiDoc::openapi())
      )
    } else {
      app_builder
    };

    app_builder
      .service(
        web::scope("/api")
          .service(
            web::scope("/ping")
              .service(ping::say_pong)
          )
          .service(
            web::scope("/auth")
              .service(auth::login)
              .service(auth::register)
          )
          .service(
            web::scope("/user")
              .service(user::get_by_id)
              .service(user::get_list)
              .service(user::update_suspended)
              .wrap(JwtAuth::new(vec![UserRole::Admin, UserRole::User], app_state.clone()))
              // scope-wide middleware: protect the scope with JwtAuth
          )
          .service(
            web::scope("/book")
              .service(book::get_list)
              .service(book::get_by_id)
              .service(book::add_one)
              .service(book::delete_one)
              .wrap(JwtAuth::new(vec![UserRole::Admin, UserRole::User], app_state.clone()))
          )
          .service(
            web::scope("/author")
              .service(author::get_list)
              .service(author::get_by_id)
              .service(author::add_one)
              .service(author::delete_one)
              .wrap(JwtAuth::new(vec![UserRole::Admin, UserRole::User], app_state.clone()))
          )
          // log requests and responses
      )
      .wrap(Logger::default())
  })
    .bind((host, port))
    .unwrap()
    .run()
}