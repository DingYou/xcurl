use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub fn load_config() -> Config {
    let home = env!("HOME");
    let conf_path = format!("{}/.xcurl/conf.json", home);
    let path_name = Path::new(&conf_path);
    let display = path_name.display();
    println!("load config from: {}", display);

    let file = match File::open(&path_name) {
        Ok(file) => file,
        Err(e) => {
            panic!("{} not found, {}", display, e);
        }
    };

    let reader = BufReader::new(file);
    let conf: Config = match serde_json::from_reader(reader) {
        Ok(conf) => conf,
        Err(e) => {
            panic!("read err, {}", e);
        }
    };
    return conf;
}

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
