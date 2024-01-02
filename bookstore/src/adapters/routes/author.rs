use actix_web::{http, HttpResponse, Responder, web};
use uuid::Uuid;

use crate::adapters::middleware::jwt::JwtClaims;
use crate::application::dto::request::author::{AddAuthorReq, GetAuthorListReq};
use crate::application::entities::user::UserRole;
use crate::application::services::author::{AuthorAddResult, AuthorDeleteResult, AuthorFetchResult, AuthorListFetchResult};
use crate::application::state::app_state::AppState;


#[utoipa::path(
  get,
  tag = "Авторы",
  context_path = "/api/author",
  params(
    ("page" = u32, Query, description = "Индекс страницы.", example = 0),
    ("size" = u32, Query, description = "Размер одной страницы.", minimum = 1, example = 20),
  ),
  responses(
    (status = OK, body = AuthorListResp),
  ),
  security(
    ("jwt_auth" = [])
  )
)]
#[get("")]
pub async fn get_list(
  state: web::Data<AppState>,
  query: web::Query<GetAuthorListReq>,
) -> impl Responder
{
  match state.author_service.get_list(query.0).await {
    AuthorListFetchResult::Ok(authors) => (web::Json(Some(authors)), http::StatusCode::OK),
    AuthorListFetchResult::UnexpectedError(_) => (web::Json(None), http::StatusCode::INTERNAL_SERVER_ERROR),
  }
}

#[utoipa::path(
  get,
  tag = "Авторы",
  context_path = "/api/author",
  params(
    ("id" = Uuid, Path, description = "Идентификатор автора."),
  ),
  responses(
    (status = OK, body = FullAuthorResp),
    (status = NOT_FOUND, description = "Автор с таким идентификатором не найден."),
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
  match state.author_service.get_by_id(&query.0).await {
    AuthorFetchResult::Ok(author) => (web::Json(Some(author)), http::StatusCode::OK),
    AuthorFetchResult::NotFound => (web::Json(None), http::StatusCode::NOT_FOUND),
    AuthorFetchResult::UnexpectedError(_) => (web::Json(None), http::StatusCode::INTERNAL_SERVER_ERROR),
  }
}

#[utoipa::path(
  delete,
  tag = "Авторы",
  context_path = "/api/author",
  params(
    ("id" = Uuid, Path, description = "Идентификатор автора."),
  ),
  responses(
    (status = OK, description = "Автор удален."),
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

  match state.author_service.delete_one(&path.0).await {
    AuthorDeleteResult::Ok => HttpResponse::new(http::StatusCode::OK),
    AuthorDeleteResult::UnexpectedError(_) => HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR),
  }
}

#[utoipa::path(
  post,
  tag = "Авторы",
  context_path = "/api/author",
  request_body = AddAuthorReq,
  responses(
    (status = CREATED, description = "Автор добавлен."),
    (status = FORBIDDEN, description = "Недостаточно прав для выполнения действия."),
  ),
  security(
    ("jwt_auth" = [])
  )
)]
#[post("")]
pub async fn add_one(
  state: web::Data<AppState>,
  data: web::Json<AddAuthorReq>,
  auth_claims: web::ReqData<JwtClaims>,
) -> impl Responder
{
  if auth_claims.role != UserRole::Admin {
    return HttpResponse::new(http::StatusCode::FORBIDDEN)
  }

  match state.author_service.add_one(data.0).await {
    AuthorAddResult::Created => HttpResponse::new(http::StatusCode::CREATED),
    AuthorAddResult::UnexpectedError(_) => HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR),
  }
}