use postgres::{Client, Error, NoTls};
use std::env;

pub fn get_pg_connection_string() -> std::string::String {
    let user = &env::var("pg_user")
        .expect("user not found in the environment variable");
    let user_password = &env::var("pg_user_password")
        .expect("user_password not found in the environment variable");
    let host_name = &env::var("pg_host_name")
        .expect("host_name not found in the environment variable");
    let port = &env::var("pg_port")
        .expect("port not found in the environment variable");
    let database = &env::var("pg_database")
        .expect("database not found in the environment variable");

    format!("postgresql://{user}:{user_password}@{host_name}:{port}/{database}")
}

pub fn get_db_schema() -> std::string::String {
    env::var("pg_schema").unwrap_or_else(|_| "public".to_string())
}

pub fn create_connection_client() -> Result<Client, Error> {
    let mut client = Client::connect(&get_pg_connection_string(), NoTls);
    client
}
