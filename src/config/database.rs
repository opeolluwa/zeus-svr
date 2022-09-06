use mongodb::{options::ClientOptions, Client};
use std::env;

pub async fn mongodb() -> mongodb::Database {
    let database_uri =
        env::var("DATABASE_URI").unwrap_or_else(|_| String::from("mongodb://localhost:27017"));
    let database_name = env::var("DATABASE_NAME").unwrap_or_else(|_| String::from("zeus"));

    // Get a handle to the cluster  and Ping the server to see if you can connect to the cluster
    let database_client_options = ClientOptions::parse(&database_uri)
        .await
        .expect("err connecting  to database");

    let database_client =
        Client::with_options(database_client_options).expect("unable to connect to db");
    //return database
    database_client.database(&database_name)
}
