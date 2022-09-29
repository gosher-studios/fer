use std::{env, process::Command};

fn main() {
    match env::args().nth(1).as_deref() {
        Some("--upload") | Some("-u") => upload(&env::args().nth(2).unwrap()),
        Some("--help") | Some("-h") => help(),
        Some(_) => println!("nonextistent argument do --help or -h to find out commands"),
        None => println!("you donkey no arguments provided"),
    }
}

fn upload(argu: &String) {
    let out = Command::new("curl")
        .arg(format!("http://localhost:4040/up/{}", argu))
        .arg("--data-binary")
        .arg(format!("@{}", argu))
        .output()
        .unwrap();
    println!(
        "uploaded file {} to {}",
        argu,
        String::from_utf8(out.stdout).unwrap()
    );
}

fn help() {
    println!("Floppa File transFER");
    println!("upload: aliases --upload, -u");
    println!("usage: fer -u example.txt");
}
