pub fn get_db_url() -> String {
  format!(
    "postgres://{}:{}@{}:{}/{}",
    std::env::var("APP_DATABASE_USER").unwrap_or("postgres".to_string()),
    std::env::var("APP_DATABASE_PASS").unwrap_or("postgres".to_string()),
    std::env::var("APP_DATABASE_HOST").unwrap_or("localhost".to_string()),
    std::env::var("APP_DATABASE_PORT").unwrap_or("5432".to_string())
      .parse::<u16>()
      .unwrap(),
    std::env::var("APP_DATABASE_NAME").unwrap_or("postgres".to_string()),
  )
}