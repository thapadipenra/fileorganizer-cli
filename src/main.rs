use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;
use std::path::Path;

fn main() -> io::Result<()> {
    // Define categories and their regex patterns
    let mut categories: HashMap<&str, Vec<&str>> = HashMap::new();
    categories.insert(
        "Foundation of Science",
        vec![
            r"science",
            r"python",
            r"physics",
            r"chemistry",
            r"biology",
            r"pytorch",
        ],
    );
    categories.insert("Mathematics and Statistics", vec![r"math", r"statistics"]);
    categories.insert(
        "Foundation of Informatics",
        vec![r"informatics", r"computerscience", r"computer"],
    );
    categories.insert(
        "Business Administration and Accounting",
        vec![
            r"business",
            r"accounting",
            r"rust",
            r"finance",
            r"economics",
            r"cli",
            r"commands",
            r"linux",
        ],
    );
    categories.insert("German Language", vec![r"german", r"deutsch"]);
    categories.insert(
        "Foundation of Medicine",
        vec![
            r"medicine",
            r"health",
            r"doctor",
            r"hospital",
            r"medical",
            r"terminology",
            r"anatomy",
        ],
    );

    // Get the directory path
    let dir_path = env::args().nth(1).expect("Please provide a directory path");
    let dir_path = Path::new(&dir_path);

    // Read the directory
    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let file_name = entry.file_name();
        let file_name_str = file_name.to_str().unwrap().to_lowercase();

        // Match file name to categories
        for (category, patterns) in &categories {
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
