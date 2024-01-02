use actix_web::{http, Responder, web};
use uuid::Uuid;
use crate::adapters::middleware::jwt::JwtClaims;

use crate::application::dto::request::user::{GetUserListReq, UpdateSuspendedReq};
use crate::application::entities::user::UserRole;
use crate::application::services::user::{UserFetchResult, UserListFetchResult, UserUpdateSuspendedResult};
use crate::application::state::app_state::AppState;


#[utoipa::path(
  get,
  tag = "Пользователи",
  context_path = "/api/user",
  params(
    ("page" = u32, Query, description = "Индекс страницы.", example = 0),
    ("size" = u32, Query, description = "Размер одной страницы.", minimum = 1, example = 20),
  ),
  responses(
    (status = OK, body = UserListResp),
  ),
  security(
    ("jwt_auth" = [])
  )
)]
#[get("")]
pub async fn get_list(
  state: web::Data<AppState>,
  query: web::Query<GetUserListReq>,
) -> impl Responder
{
  match state.user_service.get_list(query.0).await {
    UserListFetchResult::Ok(users) => (web::Json(Some(users)), http::StatusCode::OK),
    UserListFetchResult::UnexpectedError(_) => (web::Json(None), http::StatusCode::INTERNAL_SERVER_ERROR),
  }
}

#[utoipa::path(
  get,
  tag = "Пользователи",
  context_path = "/api/user",
  params(
    ("id" = Uuid, Path, description = "Идентификатор пользователя."),
  ),
  responses(
    (status = OK, body = FullUserResp),
    (status = NOT_FOUND, description = "Пользователь с таким идентификатором не найден."),
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
  match state.user_service.get_by_id(&query.into_inner().0).await {
    UserFetchResult::Ok(user) => (web::Json(Some(user)), http::StatusCode::OK),
    UserFetchResult::NotFound => (web::Json(None), http::StatusCode::NOT_FOUND),
    UserFetchResult::UnexpectedError(_) => (web::Json(None), http::StatusCode::INTERNAL_SERVER_ERROR),
  }
}

#[utoipa::path(
  put,
  tag = "Пользователи",
  context_path = "/api/user",
  params(
    ("id" = Uuid, Path, description = "Идентификатор пользователя."),
  ),
  request_body = UpdateSuspendedReq,
  responses(
    (status = OK, body = FullUserResp),
    (status = FORBIDDEN, description = "Недостаточно прав для выполнения действия."),
    (status = NOT_FOUND, description = "Пользователь с таким идентификатором не найден."),
  ),
  security(
    ("jwt_auth" = [])
  )
)]
#[put("/{id}/suspend")]
pub async fn update_suspended(
  state: web::Data<AppState>,
  path: web::Path<(Uuid, )>,
  data: web::Json<UpdateSuspendedReq>,
  auth_claims: web::ReqData<JwtClaims>,
) -> impl Responder
{
  if auth_claims.role != UserRole::Admin {
    return (web::Json(None), http::StatusCode::FORBIDDEN)
  }

  match state.user_service.update_suspended(&path.into_inner().0, data.0).await {
    UserUpdateSuspendedResult::Ok(user) => (web::Json(Some(user)), http::StatusCode::OK),
    UserUpdateSuspendedResult::NotFound => (web::Json(None), http::StatusCode::NOT_FOUND),
    UserUpdateSuspendedResult::UnexpectedError(_) => (web::Json(None), http::StatusCode::INTERNAL_SERVER_ERROR),
  }
}