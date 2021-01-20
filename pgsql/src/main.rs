extern crate postgres;

use postgres::{Client, NoTls};

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut client = Client::connect("postgresql://postgres:XXXXXXX@localhost", NoTls)?; 
    client.batch_execute(
        "CREATE TABLE person (
                id      SERIAL PRIMARY KEY,
                name    TEXT NOT NULL,
                data    BYTEA
        )",
    )?;

    let name = "Ferris";
    let data = None::<&[u8]>;
    client.execute(
        "INSERT INTO person (name, data) VALUES ($1, $2)",
        &[&name, &data],
    )?;

    for row in client.query("SELECT id, name, data FROM person", &[])? {
        let id: i32 = row.get(0);
        let name: &str = row.get(1);
        let data: Option<&[u8]> = row.get(2);

        println!("found person: {} {} {:?}", id, name, data);
    }

    Ok(())
}
