use clap::{Parser, Subcommand};
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{Read, Write};
use std::collections::HashMap;

const DB_FILE: &str = "kvstore.db";

/// Universal Key-Value storage (binary based)
#[derive(Parser)]
#[command(
    author = "Helamri",
    version = "1.0",
    about = "A simple binary Key-Value store",
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {

    /// Store a key with associated data (as binary)
    ///
    /// Example:
    ///   store-cli set username "Hamza"
    Set {
        /// Key name
        key: String,

        /// Value as string (will be stored as bytes)
        value: String,
    },

    /// Get a stored value by key
    ///
    /// Example:
    ///   store-cli get username
    Get {
        /// Key to retrieve
        key: String,
    },

    /// List all stored keys
    List,

    /// Delete a specific key
    Delete {
        key: String,
    },

    /// Reset the database
    Reset,
}

#[derive(Serialize, Deserialize, Debug)]
struct KVStore {
    data: HashMap<String, Vec<u8>>,
}

/// Load the database from file
fn load_db() -> KVStore {
    if let Ok(mut file) = File::open(DB_FILE) {
        let mut buffer = Vec::new();
        if file.read_to_end(&mut buffer).is_ok() {
            if let Ok(decoded) = bincode::deserialize::<KVStore>(&buffer) {
                return decoded;
            }
        }
    }

    KVStore {
        data: HashMap::new(),
    }
}

/// Save the database to file
fn save_db(db: &KVStore) {
    let encoded = bincode::serialize(db).unwrap();
    let mut file = File::create(DB_FILE).unwrap();
    file.write_all(&encoded).unwrap();
}

fn main() {
    let cli = Cli::parse();
    let mut db = load_db();

    match cli.command {
        Commands::Set { key, value } => {
            db.data.insert(key.clone(), value.into_bytes());
            save_db(&db);
            println!("Stored key '{}'", key);
        }

        Commands::Get { key } => {
            if let Some(v) = db.data.get(&key) {
                let as_text = String::from_utf8_lossy(v);
                println!("{} = {}", key, as_text);
            } else {
                println!("Key not found");
            }
        }

        Commands::List => {
            println!("Stored keys:");
            for key in db.data.keys() {
                println!("- {}", key);
            }
        }

        Commands::Delete { key } => {
            if db.data.remove(&key).is_some() {
                save_db(&db);
                println!("Deleted key '{}'", key);
            } else {
                println!("Key not found");
            }
        }

        Commands::Reset => {
            db.data.clear();
            save_db(&db);
            println!("Database cleared");
        }
    }
}
