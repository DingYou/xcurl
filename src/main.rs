use std::env;
use std::mem;
use std::process::Command;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    let url: &String = &args[0].clone();
    let new_url = url
        .replace(
            "https://www.dingyou.tech/api/common",
            "http://127.0.0.1:5000/api",
        )
        .replace(
            "http://localhost:10706/api/common",
            "http://127.0.0.1:5000/api",
        );
    let old = mem::replace(&mut args[0], new_url);
    println!("original url: {}", old);
    println!("replaced url: {}", args[0]);
    let out = Command::new("curl")
        .args(args)
        .output()
        .expect("execute failed");
    let output_str = String::from_utf8_lossy(&out.stdout);
    println!("{}", output_str);
    let ouput_err = String::from_utf8_lossy(&out.stderr);
    println!("{}", ouput_err);
}
