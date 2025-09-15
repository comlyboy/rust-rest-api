use mongodb::{Client, Database, options::ClientOptions};

pub struct Databases {
  pub main: Database,
  pub analytics: Database,
  pub logs: Database,
}

pub async fn initialize_dbs() -> Databases {
  let conn_str = "mongodb://localhost:27017";

  let options = ClientOptions::parse(conn_str)
    .await
    .expect("Failed to parse MongoDB options");

  let client = Client::with_options(options).expect("Failed to initialize MongoDB client");

  Databases {
    main: client.database("my_app_db"),
    analytics: client.database("analytics_db"),
    logs: client.database("logs_db"),
  }
}
