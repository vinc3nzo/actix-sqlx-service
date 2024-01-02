use actix_web::Responder;


#[get("")]
pub async fn say_pong() -> impl Responder
{
  "pong"
}
