use std::{env, fs::OpenOptions, io::{self, Write}};

pub fn save_state(content: Vec<u8>) -> io::Result<()> {
    match env::current_dir() {
        Ok(env_path) => {
            match OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .append(false)
                .open(env_path.join(".mintl"))
            {
                Ok(mut file) => {
                    if let Err(e) = file.write(&content) {
                        eprintln!("Failed to write to file: {}", e);
                    }
                }
                Err(e) => {
                    eprintln!("Failed to open or create save file: {}", e);
                }
            };
        }
        Err(e) => {
            eprintln!("Failed to find working directory: {}", e)
        }
    }
    Ok(())
}

