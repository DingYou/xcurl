use serde::Deserialize;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub store_history: bool,
    pub replaces: Vec<Replace>,
}

#[derive(Debug, Deserialize)]
pub struct Replace {
    pub from: String,
    pub to: String,
}

pub fn load_config() -> Config {
    let config_path = get_config_path();
    println!("load config from: {}", config_path.as_path().display());

    let file = match File::open(&config_path) {
        Ok(file) => file,
        Err(e) => {
            panic!("conf.json not found, {}", e);
        }
    };

    let reader = BufReader::new(file);
    match serde_json::from_reader(reader) {
        Ok(conf) => conf,
        Err(e) => {
            panic!("read err, {}", e);
        }
    }
}

fn get_config_path() -> PathBuf {
    // find conf.json in ., ${xcurl}/,  ~/.xcurl/
    let current_path = env::current_dir().unwrap();
    let current_path_conf = current_path.with_file_name("conf.json");
    if current_path_conf.exists() {
        return current_path_conf;
    }

    let current_exe_path = env::current_exe().unwrap();
    let current_exe_path_conf = current_exe_path.with_file_name("conf.json");
    if current_exe_path_conf.exists() {
        return current_exe_path_conf;
    }

    let home = env!("HOME");
    let mut home_path_conf = PathBuf::from(home);
    home_path_conf.push(".xcurl");
    home_path_conf.push("conf.json");
    if home_path_conf.exists() {
        return home_path_conf;
    }
    panic!("None conf.json found in current path, xcurl install path or ~/.xcurl/");
}
