use utoipa::{Modify, OpenApi};
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};


#[derive(OpenApi)]
#[openapi(
  paths(
    bookstore::adapters::routes::auth::register,
    bookstore::adapters::routes::auth::login,

    bookstore::adapters::routes::user::get_list,
    bookstore::adapters::routes::user::get_by_id,
    bookstore::adapters::routes::user::update_suspended,

    bookstore::adapters::routes::book::get_list,
    bookstore::adapters::routes::book::get_by_id,
    bookstore::adapters::routes::book::delete_one,
    bookstore::adapters::routes::book::add_one,

    bookstore::adapters::routes::author::get_list,
    bookstore::adapters::routes::author::get_by_id,
    bookstore::adapters::routes::author::delete_one,
    bookstore::adapters::routes::author::add_one,
  ),
  components(
    schemas(
      bookstore::application::dto::response::user::TokenResp,
      bookstore::application::dto::response::user::UserListResp,
      bookstore::application::dto::response::user::FullUserResp,

      bookstore::application::dto::response::author::FullAuthorResp,
      bookstore::application::dto::response::author::MinAuthorResp,
      bookstore::application::dto::response::author::AuthorListResp,

      bookstore::application::dto::response::book::FullBookResp,
      bookstore::application::dto::response::book::MinBookResp,
      bookstore::application::dto::response::book::BookListResp,

      bookstore::application::dto::request::user::RegisterReq,
      bookstore::application::dto::request::user::LoginReq,
      bookstore::application::dto::request::user::UpdateSuspendedReq,

      bookstore::application::dto::request::author::AddAuthorReq,
      bookstore::application::dto::request::book::AddBookReq,

      bookstore::application::entities::user::UserRole,
    )
  ),
  modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

// security scheme for interactive docs to correctly handle the JWT authorization
struct SecurityAddon;

impl Modify for SecurityAddon {
  fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
    let components = openapi.components.as_mut().unwrap();
    components.add_security_scheme(
      "jwt_auth",
      SecurityScheme::Http(
        HttpBuilder::new()
          .scheme(HttpAuthScheme::Bearer)
          .bearer_format("JWT")
          .build()
      ),
    )
  }
}
