mod config;

use crate::config::load_config;
use std::env;
use std::mem;
use std::process::Command;

fn main() {
    let conf = load_config();
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    let (url, index) = get_url_and_index(&args);
    let mut url = url;
    let replaces = conf.replaces;
    replaces
        .iter()
        .for_each(|f| url = url.replace(&f.from, &f.to));
    let old = mem::replace(&mut args[index], url.to_string());
    println!("original url: {}", old);
    println!("replaced url: {}", args[index]);
    let out = match Command::new("curl").args(args).output() {
        Ok(out) => out,
        Err(e) => panic!("{}", e),
    };
    let output_str = String::from_utf8_lossy(&out.stdout);
    println!("{}", output_str);
    let output_err = String::from_utf8_lossy(&out.stderr);
    println!("{}", output_err);
}

fn get_url_and_index(args: &Vec<String>) -> (String, usize) {
    for (index, arg) in args.iter().enumerate() {
        if arg.starts_with("-") {
            continue;
        }
        if arg.starts_with("'http") || arg.starts_with("\"http") || arg.starts_with("http") {
            return (arg.to_string(), index);
        }
    }
    panic!("url not found");
}
