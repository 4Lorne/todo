use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{stdin, stdout, Read, Write};
use toml::de::from_str;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub directory: String,
}

pub fn read_config() -> Config {
    if !std::path::Path::new("config.toml").exists() {
        create_config();
    }
    let mut file = File::open("config.toml").expect("Unable to open config file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read config file");
    from_str(&contents).expect("Unable to parse config file")
}

fn create_config() {
    println!(
        "You have not set a directory yet. 
    Please enter a directory to store your files in or leave empty to use the default directory."
    );
    let mut path = String::new();

    print!("Directory: ");

    let _ = stdout().flush();
    stdin().read_line(&mut path).expect("Error");

    if path.trim().is_empty() {
        let config = Config {
            directory: String::from(""),
        };

        let toml = toml::to_string(&config).expect("Unable to serialize config");
        std::fs::write("config.toml", toml).expect("Unable to write config file");
        return;
    } else {
        let config = Config {
            directory: path.trim().to_string(),
        };

        let toml = toml::to_string(&config).expect("Unable to serialize config");
        std::fs::write("config.toml", toml).expect("Unable to write config file");
        return;
    }
}

pub fn config_exists() -> bool {
    std::path::Path::new("config.toml").exists()
}
