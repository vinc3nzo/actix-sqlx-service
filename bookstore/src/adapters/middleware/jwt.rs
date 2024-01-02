use std::future::{ready, Ready};
use std::str::FromStr;
use std::sync::Arc;
use std::time::SystemTime;
use actix_web::{dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}, Error, HttpMessage, web};
use actix_web::error::{ErrorForbidden, ErrorUnauthorized, ErrorInternalServerError, ErrorNotFound};
use serde::{Deserialize, Serialize};
use futures_util::future::LocalBoxFuture;

use jwt;
use jwt::{SignWithKey, VerifyWithKey};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use uuid::Uuid;

use crate::application::entities::user::UserRole;
use crate::application::services::user::UserFetchResult;
use crate::application::state::app_state::AppState;

const JWT_EXPIRATION_TIME: u64 = 60 * 60 * 24 * 30; // 1 month

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtClaims {
  pub id: String,
  pub role: UserRole,
  pub exp: u64,
}

impl JwtClaims {
  pub fn new(id: Uuid, role: UserRole) -> Self {
    Self {
      id: id.to_string(),
      role,
      exp: SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("going back in time, huh?")
        .as_secs() + JWT_EXPIRATION_TIME,
    }
  }

  pub fn from_token(token: String) -> Result<Self, jwt::Error> {
    let secret = std::env::var("APP_SECRET").unwrap();
    let key: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes()).unwrap();
    return token.verify_with_key(&key);
  }

  pub fn to_token(self) -> String {
    let secret = std::env::var("APP_SECRET").unwrap();
    let key: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes()).unwrap();
    return self.sign_with_key(&key).unwrap();
  }
}

pub struct JwtAuth {
  /// roles, which have access to the resource
  roles: Vec<UserRole>,

  /// global application state (to be able to use services)
  app_state: web::Data<AppState>,
}

impl JwtAuth {
  pub fn new(roles_permitted: Vec<UserRole>, app_state: web::Data<AppState>) -> Self {
    Self {
      roles: roles_permitted,
      app_state,
    }
  }
}

impl<S, B> Transform<S, ServiceRequest> for JwtAuth
  where
    S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
  type Response = ServiceResponse<B>;
  type Error = Error;
  type Transform = JwtAuthMiddleware<S>;
  type InitError = ();
  type Future = Ready<Result<Self::Transform, Self::InitError>>;

  fn new_transform(&self, service: S) -> Self::Future {
    ready(Ok(JwtAuthMiddleware {
      service: Arc::new(service),
      roles: self.roles.clone(),
      app_state: self.app_state.clone(),
    }))
  }
}

pub struct JwtAuthMiddleware<S> {
  service: Arc<S>,
  roles: Vec<UserRole>,
  app_state: web::Data<AppState>,
}

impl<S, B> Service<ServiceRequest> for JwtAuthMiddleware<S>
  where
    S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
  type Response = ServiceResponse<B>;
  type Error = Error;
  type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

  forward_ready!(service);

  fn call(&self, req: ServiceRequest) -> Self::Future {
    // try to extract a token from the `Authorization` header
    let auth_header = match req.headers().get("Authorization") {
      Some(auth_header) => match auth_header.to_str() {
        Ok(auth_header) => auth_header,
        Err(_) => return Box::pin(async { Err(ErrorUnauthorized("")) })
      },
      None => return Box::pin(async { Err(ErrorUnauthorized("")) })
    };
    let token = match auth_header.strip_prefix("Bearer ") {
      Some(token) => token,
      None => return Box::pin(async { Err(ErrorUnauthorized("")) })
    };
    let claims = match JwtClaims::from_token(token.to_string()) {
      Ok(claims) => claims,
      Err(_) => return Box::pin(async { Err(ErrorUnauthorized("")) })
    };

    if !self.roles.contains(&claims.role) {
      return Box::pin(async { Err(ErrorForbidden("Insufficient rights for this resource.")) });
    }

    let svc = self.service.clone();
    let state = self.app_state.clone();

    Box::pin(async move {
      let user_id = match Uuid::from_str(claims.id.as_str()) {
        Ok(id) => id,
        Err(e) => {
          log::error!("Failed to extract user ID from JWT claims: {}", e);
          return Err(ErrorInternalServerError("Unexpected error. Contact the administrator."))
        },
      };

      // check if the account is suspended
      let user = match state.user_service.get_by_id(&user_id).await {
        UserFetchResult::Ok(user) => user,
        UserFetchResult::NotFound => return Err(ErrorNotFound("The associated user account could not be found.")),
        UserFetchResult::UnexpectedError(_) => return Err(ErrorInternalServerError("Unexpected error. Contact the administrator.")),
      };

      if user.suspended {
        return Err(ErrorForbidden("The user account has been suspended. Contact the administrator."));
      }

      // inject the claims into responder
      req.extensions_mut().insert(claims);

      // continue down the middleware chain
      let fut = svc.call(req);

      Ok(fut.await?)
    })
  }
}