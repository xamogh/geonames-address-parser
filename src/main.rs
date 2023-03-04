mod database;
mod parser;

use database::connection::create_connection_client;
use parser::read::read_file;
use postgres::Error;

fn main() -> Result<(), Error> {
    let mut client = create_connection_client()?;
    read_file();
    Ok(())
}
