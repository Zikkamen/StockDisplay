use postgres::{Client, NoTls};

use crate::value_store::credentials_store::CredentialsStore;

pub struct DatabaseTradeModel {
    pub first_trade:i64,
    pub num_of_trades: i32,
    pub volume_moved: i32,
    pub avg_price:i64,
    pub min_price:i64,
    pub max_price:i64,
}

pub struct PostgresClient {
    client: Client,
}

impl PostgresClient {
    pub fn new(credentials_store: CredentialsStore) -> Self {
        let client = match Client::connect(&format!(
            "host={} user={} password={}",
            credentials_store.get_token("postgres.host"),
            credentials_store.get_token("postgres.user"),
            credentials_store.get_token("postgres.password")
        ), 
        NoTls) {
            Ok(client) => client,
            Err(e) => panic!("Error creating PostgresClient {}", e),
        };

        let postgres_client:PostgresClient = PostgresClient{ client: client };

        println!("Connected to PostgreSQL");

        postgres_client
    }

    pub fn get_all_tables(&mut self) {
        let rows = match self.client.query("select table_name from information_schema.tables WHERE table_schema='public'", &[]){
            Ok(v) => v,
            Err(e) => panic!("Query is wrong: {}", e),
        };

        for row in rows {
            let time: String = row.get("table_name");

            println!("{}", time);
        }
    }
}