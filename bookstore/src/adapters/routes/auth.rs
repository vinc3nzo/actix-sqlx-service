use actix_web::{http, web, Responder};

use crate::application::state::app_state::AppState;
use crate::application::dto::request::user::{LoginReq, RegisterReq};
use crate::application::dto::response::user::TokenResp;
use crate::application::services::auth::RegistrationError;

#[utoipa::path(
  post,
  tag = "Аутентификация",
  context_path = "/api/auth",
  request_body = RegisterReq,
  responses(
    (status = CREATED, body = TokenResp),
    (status = BAD_REQUEST, description = "Неверный формат полей."),
    (status = CONFLICT, description = "Пользователь с такми псевдонимом уже существует."),
  )
)]
#[post("/register")]
pub async fn register(
  state: web::Data<AppState>,
  data: web::Json<RegisterReq>,
) -> impl Responder
{
  match state.auth_service.register(data.0).await {
    Ok(token) => (web::Json(Some(TokenResp { token })), http::StatusCode::CREATED),
    Err(e) => match e {
      RegistrationError::AlreadyExists => (web::Json(None), http::StatusCode::CONFLICT),
      RegistrationError::BadRequest => (web::Json(None), http::StatusCode::BAD_REQUEST),
      RegistrationError::UnexpectedError => (web::Json(None), http::StatusCode::INTERNAL_SERVER_ERROR)
    }
  }
}

#[utoipa::path(
  post,
  tag = "Аутентификация",
  context_path = "/api/auth",
  request_body = LoginReq,
  responses(
    (status = OK, body = TokenResp),
    (status = UNAUTHORIZED, description = "Не удалось авторизоваться."),
  )
)]
#[post("/login")]
pub async fn login(
  state: web::Data<AppState>,
  data: web::Json<LoginReq>,
) -> impl Responder
{
  match state.auth_service.login(data.0).await {
    Ok(token) => match token {
      None => (web::Json(None), http::StatusCode::UNAUTHORIZED),
      Some(token) => (web::Json(Some(TokenResp { token })), http::StatusCode::OK)
    },
    Err(_) => (web::Json(None), http::StatusCode::INTERNAL_SERVER_ERROR),
  }
}