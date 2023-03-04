use postgres::{Client, Error, NoTls};
use std::env;

fn get_pg_connection_string() -> std::string::String {
    let user = &env::var("user").unwrap();
    let user_password = &env::var("user_password").unwrap();
    let host_name = &env::var("host_name").unwrap();
    let port = &env::var("port").unwrap();
    let database = &env::var("database").unwrap();
    let schema = &env::var("schema").unwrap();

    format!("postgresql://{user}:{user_password}@{host_name}:{port}/{database}?currentSchema={schema}")
}

fn main() -> Result<(), Error> {
    let mut client = Client::connect(&get_pg_connection_string(), NoTls)?;
    client.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS author (
            id              SERIAL PRIMARY KEY,
            name            VARCHAR NOT NULL,
            country         VARCHAR NOT NULL
            )
    ",
    )?;

    client.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS book  (
            id              SERIAL PRIMARY KEY,
            title           VARCHAR NOT NULL,
            author_id       INTEGER NOT NULL REFERENCES author
            )
    ",
    )?;

    Ok(())
}
