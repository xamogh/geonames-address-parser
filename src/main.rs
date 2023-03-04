use postgres::Error;
mod database;
use database::connection::{create_connection_client, get_db_schema};

fn main() -> Result<(), Error> {
    let mut client = create_connection_client()?;
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
