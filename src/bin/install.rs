use std::path::Path;
use std::{env, fs, io};

fn install() -> io::Result<()> {
    // Get the target migrations directory from environment or use default
    let project_migrations_dir = env::var("PROJECT_MIGRATIONS_DIR")
        .unwrap_or_else(|_| "migrations".to_string());
    let project_migrations_path = Path::new(&project_migrations_dir);

    // Check if the migrations directory exists
    if !project_migrations_path.exists() {
        println!("Creating migrations directory: {}", project_migrations_dir);
        fs::create_dir_all(project_migrations_path)?;
    }

    // Path to your crate's migrations directory
    let crate_migrations_dir = Path::new("migrations");

    // Copy files from the crate's migrations directory to the project
    if crate_migrations_dir.exists() {
        for entry in fs::read_dir(crate_migrations_dir)? {
            let entry = entry?;
            let entry_path = entry.path();
            if entry_path.is_file() {
                let destination = project_migrations_path.join(entry.file_name());
                println!("Copying {} to {}", entry_path.display(), destination.display());
                fs::copy(entry_path, destination)?;
            }
        }
    } else {
        eprintln!("No migrations found in the crate.");
    }

    Ok(())
}

fn main() {
    if let Err(e) = install() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
