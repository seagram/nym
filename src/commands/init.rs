use std::fs;
use std::fs::File;
use std::env;
use xdg;

pub fn init(){
    // TODO: Migrate .unwrap() to match statement
    // TODO: Shorten directory path to '~' prefix
    let xdg_dirs = xdg::BaseDirectories::with_prefix("nym");
    let config_home = xdg_dirs.get_config_home().unwrap();
    if !config_home.exists() {
        fs::create_dir_all(&config_home).unwrap();
        println!("{} created.", config_home.to_string_lossy());
    } else {
        println!("{} already exists.", config_home.to_string_lossy())
    }

    let config_path = xdg_dirs.place_config_file("config.toml").expect("cannot create config.toml");
    if !config_path.exists() {
        _ = File::create_new(&config_path);
        println!("{} created.", config_path.to_string_lossy());
    } else {
        println!("{} already exists.", config_path.to_string_lossy())
    }

    let alias_path = xdg_dirs.place_config_file("aliases.sh").expect("cannot create aliases.sh");
    if !alias_path.exists() {
        _ = File::create_new(&alias_path);
        println!("{} created.", alias_path.to_string_lossy());
    } else {
        println!("{} already exists.", alias_path.to_string_lossy())
    }

    match env::var("SHELL") {
        Ok(shell_path) if shell_path.contains("zsh") => {
            println!("\nAdd the following to your .zshrc file:\n  source {}", alias_path.to_string_lossy());
        }
        Ok(shell_path) if shell_path.contains("bash") => {
            println!("\nAdd the following to your .bashrc file:\n  source {}", alias_path.to_string_lossy());
        }
        Ok(shell_path) => {
            println!("Unknown shell: {}", shell_path);
        }
        Err(e) => {
            println!("Could not read $SHELL environment variable: {}", e);
        }
    }
}