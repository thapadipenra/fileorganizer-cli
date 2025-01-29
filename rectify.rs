use regex::Regex;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct Config {
    categories: HashMap<String, Vec<String>>,
}

fn main() -> io::Result<()> {
    // Default configuration
    let default_config = r#"
categories:
  Foundation of Science:
    - science
    - python
    - physics
    - chemistry
    - biology
    - pytorch
  Mathematics and Statistics:
    - math
    - statistics
  Foundation of Informatics:
    - informatics
    - computerscience
    - computer
  Business Administration and Accounting:
    - business
    - accounting
    - rust
    - finance
    - economics
    - cli
    - commands
    - linux
  German Language:
    - german
    - deutsch
  Foundation of Medicine:
    - medicine
    - health
    - doctor
    - hospital
    - medical
    - terminology
    - anatomy
"#;

    // Read the configuration file
    let config_path = "config.yaml";
    let config: Config = if Path::new(config_path).exists() {
        let file = File::open(config_path).expect("Unable to open config file");
        serde_yaml::from_reader(file).expect("Unable to parse config file")
    } else {
        serde_yaml::from_str(default_config).expect("Unable to parse default config")
    };

    // Get the directory path
    let dir_path = env::args().nth(1).expect("Please provide a directory path");
    let dir_path = Path::new(&dir_path);

    // Read the directory
    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let file_name = entry.file_name();
        let file_name_str = file_name.to_str().unwrap().to_lowercase();

        // Match file name to categories
        for (category, patterns) in &config.categories {
            for pattern in patterns {
                let re = Regex::new(pattern).unwrap();
                if re.is_match(&file_name_str) {
                    // Create category directory if it doesn't exist
                    let category_path = dir_path.join(category);
                    if !category_path.exists() {
                        fs::create_dir(&category_path)?;
                    }

                    // Move file to category directory
                    let new_path = category_path.join(file_name.clone());
                    if entry.path().exists() {
                        fs::rename(entry.path(), new_path)?;
                        println!("Moved {} to {}", file_name_str, category);
                    } else {
                        println!("File {} not found", file_name_str);
                    }
                    break;
                }
            }
        }
    }

    Ok(())
}

// // in github aabbbbh/rectify.rs

use regex::Regex;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct Config {
    categories: HashMap<String, Vec<String>>,
}

fn main() -> io::Result<()> {
    // Default configuration
    let default_config = r#"
categories:
  Foundation of Science:
    - science
    - python
    - physics
    - chemistry
    - biology
    - pytorch
  Mathematics and Statistics:
    - math
    - statistics
  Foundation of Informatics:
    - informatics
    - computerscience
    - computer
  Business Administration and Accounting:
    - business
    - accounting
    - rust
    - finance
    - economics
    - cli
    - commands
    - linux
  German Language:
    - german
    - deutsch
  Foundation of Medicine:
    - medicine
    - health
    - doctor
    - hospital
    - medical
    - terminology
    - anatomy
"#;

    // Read the configuration file
    let config_path = "config.yaml";
    let config: Config = if Path::new(config_path).exists() {
        let file = File::open(config_path).expect("Unable to open config file");
        serde_yaml::from_reader(file).expect("Unable to parse config file")
    } else {
        serde_yaml::from_str(default_config).expect("Unable to parse default config")
    };

    // Get the directory path
    let dir_path = env::args().nth(1).expect("Please provide a directory path");
    let dir_path = Path::new(&dir_path);

    // Ensure all category directories exist
    for category in config.categories.keys() {
        let category_path = dir_path.join(category);
        if !category_path.exists() {
            if let Err(e) = fs::create_dir(&category_path) {
                eprintln!(
                    "Failed to create directory {}: {}",
                    category_path.display(),
                    e
                );
            } else {
                println!("Created directory {}", category_path.display());
            }
        }
    }

    // Process files in the directory
    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let file_name = entry.file_name();
        let file_name_str = file_name.to_str().unwrap().to_lowercase();

        // Match file name to categories
        for (category, patterns) in &config.categories {
            for pattern in patterns {
                let re = Regex::new(pattern).unwrap();
                if re.is_match(&file_name_str) {
                    // Move file to category directory
                    let category_path = dir_path.join(category);
                    let new_path = category_path.join(file_name.clone());
                    if entry.path().exists() {
                        if let Err(e) = fs::rename(entry.path(), &new_path) {
                            eprintln!(
                                "Failed to move file {} to {}: {}",
                                file_name_str,
                                category_path.display(),
                                e
                            );
                        } else {
                            println!("Moved {} to {}", file_name_str, category);
                        }
                    } else {
                        println!("File {} not found", file_name_str);
                    }
                    break;
                }
            }
        }
    }

    Ok(())
}
