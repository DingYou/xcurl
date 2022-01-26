mod config;

use crate::config::load_config;
use std::env;
use std::process::Command;

fn main() {
    let conf = load_config();
    let mut args = env::args();
    args.next();
    let args = args.map(|mut a| -> String {
        if !a.starts_with("-")
            && (a.starts_with("'http") || a.starts_with("\"http") || a.starts_with("http")) {
            println!("original url: {}", a);
            conf.replaces
                .iter()
                .for_each(|r| a = a.replace(&r.from, &r.to));
            println!("replaced url: {}", a);
        }
        return a;
    });
    let out = match Command::new("curl").args(args).output() {
        Ok(out) => out,
        Err(e) => panic!("{}", e),
    };
    let output_str = String::from_utf8_lossy(&out.stdout);
    println!("{}", output_str);
    let output_err = String::from_utf8_lossy(&out.stderr);
    println!("{}", output_err);
}
