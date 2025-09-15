use dotenv::dotenv;
use std::env;

#[derive(Debug, Clone)]
pub struct MongoConfig {
  pub uri: String,
  pub db_name: String,
}

// #[derive(Debug, Clone)]
pub struct AppConfig {
  pub env: String,
  pub mongo: MongoConfig,
}

impl AppConfig {
  pub fn from_env() -> Self {
    // Load .env only when compiled with "dotenv"
    #[cfg(feature = "dotenv")]
    dotenv().ok();

    let env_mode = env::var("APP_ENV").unwrap_or_else(|_| "development".into());

    let mongo = MongoConfig {
      uri: env::var("MONGO_URI").expect("MONGO_URI must be set"),
      db_name: env::var("MONGO_DB").expect("MONGO_DB must be set"),
    };

    Self {
      env: env_mode,
      mongo,
    }
  }
}
