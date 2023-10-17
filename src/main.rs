mod alter_seq;
pub mod config;
pub mod drop_tables;
pub mod tasker;
pub mod tasks;

use config::RdropConfig;
use postgres::NoTls;
use std::process::exit;

fn connect_to_db(connection_string: String) -> postgres::Client {
    match postgres::Client::connect(&connection_string, NoTls) {
        Ok(client) => client,
        Err(e) => {
            println!("Could not connect to database: {e}");
            exit(1);
        }
    }
}

fn main() {
    let config = match RdropConfig::new("rdrop.toml".into()) {
        Ok(config) => config,
        Err(err) => {
            println!("{err}");
            exit(1);
        }
    };

    let client = connect_to_db(config.connection_string);
    let mut tasker = tasker::Tasker::new(client);
    tasker.append_task(Box::new(drop_tables::DropTables::new()));
    tasker.run();
}
