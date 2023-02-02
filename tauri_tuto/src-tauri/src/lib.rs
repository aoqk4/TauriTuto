use data_encoding::HEXUPPER;
use postgres::{Client, Error, NoTls};
use ring::error::Unspecified;
use ring::rand::SecureRandom;
use ring::{digest, pbkdf2, rand};
use std::num::NonZeroU32;

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

    pub fn post_insert(mut cli: Client, name: &str, country: &str) -> Result<(), Error> {
        // INSERT CODE
        let mut authors = HashMap::new();

        authors.insert(String::from(name), country);

        let mut cnt = 234;

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

#[cfg(test)]
mod tests {
    use super::*;

    struct Nation {
        nationality: String,
        count: i64,
    }

    #[test]
    fn aggregate_data() -> Result<(), Error> {
        let mut client = Client::connect("postgresql://postgres:postgres@127.0.0.1/moma", NoTls)?;

        for row in client.query(
            "SELECT nationality, COUNT(nationality) AS count 
        FROM artists GROUP BY nationality ORDER BY count DESC",
            &[],
        )? {
            let (nationality, count): (Option<String>, Option<i64>) = (row.get(0), row.get(1));

            if nationality.is_some() && count.is_some() {
                let nation = Nation {
                    nationality: nationality.unwrap(),
                    count: count.unwrap(),
                };
                println!("{} {}", nation.nationality, nation.count);
            }
        }

        Ok(())
    }
    #[test]
    fn encryption_test() -> Result<(), Unspecified> {
        const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;
        let n_iter = NonZeroU32::new(100_000).unwrap();
        let rng = rand::SystemRandom::new();

        let mut salt = [0u8; CREDENTIAL_LEN];
        rng.fill(&mut salt)?;

        let password = "Guess Me If You Can!";
        let mut pbkdf2_hash = [0u8; CREDENTIAL_LEN];
        pbkdf2::derive(
            pbkdf2::PBKDF2_HMAC_SHA512,
            n_iter,
            &salt,
            password.as_bytes(),
            &mut pbkdf2_hash,
        );
        println!("Salt: {}", HEXUPPER.encode(&salt));
        println!("PBKDF2 hash: {}", HEXUPPER.encode(&pbkdf2_hash));

        let should_succeed = pbkdf2::verify(
            pbkdf2::PBKDF2_HMAC_SHA512,
            n_iter,
            &salt,
            password.as_bytes(),
            &pbkdf2_hash,
        );
        let wrong_password = "Definitely not the correct password";
        let should_fail = pbkdf2::verify(
            pbkdf2::PBKDF2_HMAC_SHA512,
            n_iter,
            &salt,
            wrong_password.as_bytes(),
            &pbkdf2_hash,
        );

        assert!(should_succeed.is_ok());
        assert!(!should_fail.is_ok());

        Ok(())
    }
}
