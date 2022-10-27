use std::process::Command;
use std::path::Path;

fn main() {
    let antlr_path = "./antlr4-4.8-2-SNAPSHOT-complete.jar";
    if !Path::new(antlr_path).exists() {
        panic!("Latest custom ANTLR build is not exists at {antlr_path}. Please download it as described at README.md");
    }

    println!("cargo:rerun-if-changed=grammar/Lang.g4");

    let _command = Command::new("java")
    .arg("-jar")
    .arg(antlr_path)
    .arg("-Dlanguage=Rust")
    .arg("-visitor")
    .arg("grammar/Lang.g4")
    .arg("-o")
    .arg("src")
    .spawn()
    .expect("antlr tool failed to start")
    .wait_with_output();
}
