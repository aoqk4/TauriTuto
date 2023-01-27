#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use postgres::{Client, Error, NoTls, Row};

    use dotenv::dotenv;

    struct Author {
        _id: i32,
        name: String,
        country: String,
    }

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

        // CREATE CODE
        // client.batch_execute(
        //     "
        //     CREATE TABLE my_schema.author (
        //         id              SERIAL PRIMARY KEY,
        //         name            VARCHAR NOT NULL,
        //         country         VARCHAR NOT NULL
        //         )
        // ",
        // )?;

        // INSERT CODE
        // let mut authors = HashMap::new();

        // authors.insert(String::from("A"), "Korea");
        // authors.insert(String::from("B"), "Japan");
        // authors.insert(String::from("C"), "China");

        // let mut cnt = 0;

        // for (key, value) in authors {
        //     let author = Author {
        //         _id: cnt,
        //         name: key.to_string(),
        //         country: value.to_string(),
        //     };

        //     client.execute(
        //         "INSERT INTO my_schema.author VALUES ($1, $2, $3)",
        //         &[&author._id, &author.name, &author.country],
        //     )?;

        //     cnt += 1;
        // }

        for row in client.query("SELECT id, name, country FROM my_schema.author", &[])? {
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
