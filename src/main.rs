mod config;

use std::env;
use std::mem;
use std::process::Command;
use crate::config::load_config;

fn main() {
    let conf = load_config();
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    let mut url = args[0].clone();
    let replaces = conf.replaces;
    replaces.iter().for_each(|f| url = url.replace(&f.from, &f.to));
    let old = mem::replace(&mut args[0],url.to_string());
    println!("original url: {}", old);
    println!("replaced url: {}", args[0]);
    let out = Command::new("curl")
        .args(args)
        .output()
        .expect("execute failed");
    let output_str = String::from_utf8_lossy(&out.stdout);
    println!("{}", output_str);
    let output_err = String::from_utf8_lossy(&out.stderr);
    println!("{}", output_err);
}
