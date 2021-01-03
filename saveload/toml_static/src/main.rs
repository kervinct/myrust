#![allow(dead_code, unused)]

use serde_derive::Deserialize;

#[derive(Deserialize)]
struct Input {
    xml_file: String,
    json_file: String,
}

#[derive(Deserialize)]
struct Redis {
    host: String,
}

#[derive(Deserialize)]
struct Sqlite {
    db_file: String,
}

#[derive(Deserialize)]
struct Postgresql {
    username: String,
    password: String,
    host: String,
    port: String,
    database: String,
}

#[derive(Deserialize)]
struct Config {
    input: Input,
    redis: Redis,
    sqlite: Sqlite,
    postgresql: Postgresql,
}

fn main() {
    let config_const_values: Config = {
        let config_path = std::env::args().nth(1).unwrap();

        let config_text = std::fs::read_to_string(&config_path).unwrap();

        toml::from_str(&config_text).unwrap()
    };

    println!(
        "[postgresql].database: {}",
        config_const_values.postgresql.database
    );

    // cargo run ../data/config.toml
}