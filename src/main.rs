use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{Read, Write};
use std::collections::HashMap;

const DB_FILE: &str = "kvstore.db";

#[derive(Serialize, Deserialize, Debug)]
struct KVStore {
    data: HashMap<String, Vec<u8>>,
}

fn load_db() -> KVStore {
    if let Ok(mut file) = File::open(DB_FILE) {
        let mut buffer = Vec::new();

        // handle corrupted file
        if file.read_to_end(&mut buffer).is_ok() {
            if let Ok(decoded) = bincode::deserialize::<KVStore>(&buffer) {
                return decoded;
            } else {
                println!("⚠️ Warning: corrupted DB file — resetting.");
            }
        }
    }
    KVStore { data: HashMap::new() }
}

fn save_db(db: &KVStore) {
    if let Ok(encoded) = bincode::serialize(db) {
        if let Ok(mut file) = File::create(DB_FILE) {
            let _ = file.write_all(&encoded);
        }
    }
}

fn main() {
    let mut db = load_db();

    // Hardcoded examples:
    db.data.insert("username".into(), b"Hamza".to_vec());
    db.data.insert("age".into(), b"33".to_vec());

    save_db(&db);

    // Read a value
    if let Some(value) = db.data.get("username") {
        println!("username = {}", String::from_utf8_lossy(value));
    }

    println!("Stored keys: {:?}", db.data.keys().collect::<Vec<&String>>());
}
