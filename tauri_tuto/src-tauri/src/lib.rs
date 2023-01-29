pub mod post_db {

    use dotenv::dotenv;
    use log::info;
    use postgres::{Client, Error, NoTls, Row};
    use std::collections::HashMap;
    struct Author {
        _id: i32,
        name: String,
        country: String,
    }

    pub fn init() -> Result<Client, Error> {
        dotenv().ok();

        let id = std::env::var("MY_DB_ID").expect("DB_ID must be set");

        let password = std::env::var("MY_DB_PASSWORD").expect("DB_PASSWORD must be set");

        let mut url = String::from("postgresql://");

        url.push_str(&id);
        url.push_str(":");
        url.push_str(&password);
        url.push_str("@localhost/tauri_tuto");

        let client = Client::connect(&url, NoTls);

        client
    }

    pub fn post_create(mut cli: Client) -> Result<(), Error> {
        // CREATE CODE
        cli.batch_execute(
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

    pub fn post_insert(mut cli: Client) -> Result<(), Error> {
        // INSERT CODE
        let mut authors = HashMap::new();

        authors.insert(String::from("D"), "Korea");
        authors.insert(String::from("E"), "Japan");
        authors.insert(String::from("F"), "China");

        let mut cnt = 4;

        for (key, value) in authors {
            let author = Author {
                _id: cnt,
                name: key.to_string(),
                country: value.to_string(),
            };

            cli.execute(
                "INSERT INTO my_schema.author VALUES ($1, $2, $3)",
                &[&author._id, &author.name, &author.country],
            )?;

            cnt += 1;
        }

        Ok(())
    }

    pub fn select_test(mut cli: Client) -> Result<(), Error> {
        // SELECT CODE
        for row in cli.query("SELECT id, name, country FROM my_schema.author", &[])? {
            let author = Author {
                _id: row.get(0),
                name: row.get(1),
                country: row.get(2),
            };

            println!("{:?}", author.name);
        }

        Ok(())
    }
}
