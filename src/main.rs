use postgres::{Client, Error, NoTls};
mod database;
use database::connection::{get_db_schema, get_pg_connection_string};

fn main() -> Result<(), Error> {
    let mut client = Client::connect(&get_pg_connection_string(), NoTls)?;
    let schema = get_db_schema();

    client.batch_execute(&format!(
        "
        CREATE TABLE IF NOT EXISTS \"{schema}\".author (
            id              SERIAL PRIMARY KEY,
            name            VARCHAR NOT NULL,
            country         VARCHAR NOT NULL
            )
        "
    ))?;

    client.batch_execute(&format!(
        "
        CREATE TABLE IF NOT EXISTS \"{schema}\".book  (
            id              SERIAL PRIMARY KEY,
            title           VARCHAR NOT NULL,
            author_id       INTEGER NOT NULL REFERENCES \"{schema}\".author
            )
        "
    ))?;

    Ok(())
}
