#[cfg(test)]
mod test {
    use postgres::{Client, Error, NoTls};

    use dotenv::dotenv;

    #[test]
    fn test() -> Result<(), Error> {
        dotenv().ok();

        let id = std::env::var("MY_DB_ID").expect("DB_ID must be set");

        let password = std::env::var("MY_DB_PASSWORD").expect("DB_PASSWORD must be set");

        let mut url = String::from("postgresql://");

        url.push_str(&id);
        url.push_str(":");
        url.push_str(&password);
        url.push_str("@localhost/tauri_tuto");

        let mut client = Client::connect(&url, NoTls)?;

        client.batch_execute(
            "
            CREATE TABLE my_schema.author (
                id              SERIAL PRIMARY KEY,
                name            VARCHAR NOT NULL,
                country         VARCHAR NOT NULL
                )
        ",
        )?;

        Ok(())
    }
}
