use postgres::{Client, Error};

struct Country {
    id: i32,
    name: String,
}

impl Country {
    fn insert(client: &mut Client, name: &str) -> Result<(), Error> {
        client.execute("INSERT INTO country (name) VALUES ($1)", &[&name])?;
        Ok(())
    }
}
