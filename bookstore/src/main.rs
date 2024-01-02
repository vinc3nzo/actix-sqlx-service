mod server;
mod db_conn;
mod api_docs;
mod init;

use actix_web;
use dotenv::dotenv;
use log4rs;

use crate::init::init;
use crate::server::create_server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv().ok();

  log4rs::init_file("log_config.yml", Default::default())
    .or(log4rs::init_file("../log_config.yml", Default::default()))
    .unwrap();

  let init_data = init().await;
  let server = create_server(&init_data.host, init_data.port, init_data.app_state);

  log::info!("The server is listening on {}:{}", init_data.host, init_data.port);
  server.await
}
