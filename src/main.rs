use clap::{Parser, Subcommand};
use serde::{Serialize, Deserialize};
use std::fs::{self, File};
use std::io::{self, Read, Write};

const DB_FILE: &str = "profiles.db";

/// Main CLI application
#[derive(Parser)]
#[command(author = "Helamri",
          version = "1.0",
          about = "A simple local binary profile manager",
          long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new profile to the local binary database.
    ///
    /// Usage:
    ///   profile-cli add "Hamza" 23
    Add {
        /// Profile name
        name: String,

        /// Profile age
        age: u32,
    },

    /// List all saved profiles.
    ///
    /// Usage:
    ///   profile-cli list
    List,

    /// Delete a profile by index.  
    /// Use `list` to see available indices.
    ///
    /// Usage:
    ///   profile-cli delete 1
    Delete {
        /// The index of the profile to delete
        index: usize,
    },

    /// Reset the entire database (delete all profiles).
    ///
    /// Usage:
    ///   profile-cli reset
    ///
    /// Warning:
    ///   This action cannot be undone.
    Reset,
}

#[derive(Serialize, Deserialize, Debug)]
struct Profile {
    name: String,
    age: u32,
}

/// Load the profiles from binary file
fn load_profiles() -> Vec<Profile> {
    if let Ok(mut file) = File::open(DB_FILE) {
        let mut buffer = Vec::new();
        if file.read_to_end(&mut buffer).is_ok() {
            if let Ok(profiles) = bincode::deserialize::<Vec<Profile>>(&buffer) {
                return profiles;
            }
        }
    }
    Vec::new()
}

/// Save profiles to binary file
fn save_profiles(profiles: &Vec<Profile>) -> io::Result<()> {
    let encoded = bincode::serialize(profiles).unwrap();
    let mut file = File::create(DB_FILE)?;
    file.write_all(&encoded)?;
    Ok(())
}

fn main() {
    let cli = Cli::parse();
    let mut profiles = load_profiles();

    match cli.command {
        Commands::Add { name, age } => {
            profiles.push(Profile { name, age });
            save_profiles(&profiles).unwrap();
            println!("Profile added successfully!");
        }

        Commands::List => {
            if profiles.is_empty() {
                println!("No profiles found.");
            } else {
                println!("Saved profiles:");
                for (i, p) in profiles.iter().enumerate() {
                    println!("{} — Name: {}, Age: {}", i, p.name, p.age);
                }
            }
        }

        Commands::Delete { index } => {
            if index < profiles.len() {
                profiles.remove(index);
                save_profiles(&profiles).unwrap();
                println!("Profile deleted successfully.");
            } else {
                println!("Invalid index.");
            }
        }

        Commands::Reset => {
            profiles.clear();
            save_profiles(&profiles).unwrap();
            println!("All profiles deleted.");
        }
    }
}
