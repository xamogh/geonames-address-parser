use postgres::{Client, Error};

struct Country {
    id: i32,
    name: String,
}

impl Country {
    fn insert_bulk(client: &mut Client, names: &[&str]) -> Result<(), Error> {
        // let mut countries = Vec::new();
        let statement =
            client.prepare("INSERT INTO country (name) VALUES ($1)")?;
        for name in names {
            client.execute(&statement, &[name])?;
        }
        Ok(())
    }
}
