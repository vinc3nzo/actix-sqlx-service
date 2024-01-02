use actix_web::{http, HttpResponse, Responder, web};
use uuid::Uuid;

use crate::adapters::middleware::jwt::JwtClaims;
use crate::application::dto::request::book::{GetBookListReq, AddBookReq};
use crate::application::entities::user::UserRole;
use crate::application::state::app_state::AppState;
use crate::application::services::book::{BookAddResult, BookDeleteResult, BookFetchResult, BookListFetchResult};


#[utoipa::path(
  get,
  tag = "Книги",
  context_path = "/api/book",
  params(
    ("page" = u32, Query, description = "Индекс страницы.", example = 0),
    ("size" = u32, Query, description = "Размер одной страницы.", minimum = 1, example = 20),
  ),
  responses(
    (status = OK, body = BookListResp),
  ),
  security(
    ("jwt_auth" = [])
  )
)]
#[get("")]
pub async fn get_list(
  state: web::Data<AppState>,
  query: web::Query<GetBookListReq>,
) -> impl Responder
{
  match state.book_service.get_list(query.0).await {
    BookListFetchResult::Ok(books) => (web::Json(Some(books)), http::StatusCode::OK),
    BookListFetchResult::UnexpectedError(_) => (web::Json(None), http::StatusCode::INTERNAL_SERVER_ERROR),
  }
}

#[utoipa::path(
  get,
  tag = "Книги",
  context_path = "/api/book",
  params(
    ("id" = Uuid, Path, description = "Идентификатор книги."),
  ),
  responses(
    (status = OK, body = FullBookResp),
    (status = NOT_FOUND, description = "Книга с таким идентификатором не найдена."),
  ),
  security(
    ("jwt_auth" = [])
  )
)]
#[get("/{id}")]
pub async fn get_by_id(
  state: web::Data<AppState>,
  query: web::Path<(Uuid, )>,
) -> impl Responder
{
  match state.book_service.get_by_id(&query.0).await {
    BookFetchResult::Ok(book) => (web::Json(Some(book)), http::StatusCode::OK),
    BookFetchResult::NotFound => (web::Json(None), http::StatusCode::NOT_FOUND),
    BookFetchResult::UnexpectedError(_) => (web::Json(None), http::StatusCode::INTERNAL_SERVER_ERROR),
  }
}

#[utoipa::path(
  delete,
  tag = "Книги",
  context_path = "/api/book",
  params(
    ("id" = Uuid, Path, description = "Идентификатор книги."),
  ),
  responses(
    (status = OK, description = "Книга удалена."),
    (status = FORBIDDEN, description = "Недостаточно прав для выполнения действия."),
  ),
  security(
    ("jwt_auth" = [])
  )
)]
#[delete("/{id}")]
pub async fn delete_one(
  state: web::Data<AppState>,
  path: web::Path<(Uuid, )>,
  auth_claims: web::ReqData<JwtClaims>,
) -> impl Responder
{
  if auth_claims.role != UserRole::Admin {
    return HttpResponse::new(http::StatusCode::FORBIDDEN)
  }

  match state.book_service.delete_one(&path.0).await {
    BookDeleteResult::Ok => HttpResponse::new(http::StatusCode::OK),
    BookDeleteResult::UnexpectedError(_) => HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR),
  }
}

#[utoipa::path(
  post,
  tag = "Книги",
  context_path = "/api/book",
  request_body = AddBookReq,
  responses(
    (status = CREATED, description = "Книга добавлена."),
    (status = NOT_FOUND, description = "Автор с таким ID не найден."),
    (status = FORBIDDEN, description = "Недостаточно прав для выполнения действия."),
  ),
  security(
    ("jwt_auth" = [])
  )
)]
#[post("")]
pub async fn add_one(
  state: web::Data<AppState>,
  data: web::Json<AddBookReq>,
  auth_claims: web::ReqData<JwtClaims>,
) -> impl Responder
{
  if auth_claims.role != UserRole::Admin {
    return HttpResponse::new(http::StatusCode::FORBIDDEN)
  }

  match state.book_service.add_one(data.0).await {
    BookAddResult::Created => HttpResponse::new(http::StatusCode::CREATED),
    BookAddResult::AuthorNotFound => HttpResponse::new(http::StatusCode::NOT_FOUND),
    BookAddResult::UnexpectedError(_) => HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR),
  }
}