use std::{
    env, fs,
    path::Path,
    process::{Command, Output},
};

use crate::parsing::parse;
use tokenization::tokenize;

mod compilation_error;
mod parsing;
mod tokenization;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("please provide a filename");
        return;
    }

    let file_path = Path::new(&args[1]);
    let file_string = fs::read_to_string(file_path).expect("Unable to read file!");

    match tokenize(&file_string) {
        Ok(mut tokens) => match parse(&mut tokens) {
            Ok(output) => assemble(output),
            Err(e) => eprintln!("{}", e),
        },
        Err(e) => eprintln!("{}", e),
    }
}

fn assemble(code: String) {
    fs::create_dir_all("out").expect("failed to create dir");
    fs::write("out/assembly.asm", code).expect("error writing to file");

    let nasm_output = Command::new("nasm")
        .arg("-felf64")
        .arg("out/assembly.asm")
        .arg("-oout/assembly.o")
        .output()
        .expect("nasm command failed to start");
    print_output(nasm_output);
    let ld_output = Command::new("ld")
        .arg("out/assembly.o")
        .arg("-oout/assembly")
        .output()
        .expect("ld command failed to start");
    print_output(ld_output);
    let executable_output = Command::new("out/assembly")
        .output()
        .expect("failed to run executable");
    print_output(executable_output);
}

fn print_output(output: Output) {
    println!(
        "status: {}\n stdout: {}\n stderr: {}",
        output.status,
        String::from_utf8(output.stdout).unwrap(),
        String::from_utf8(output.stderr).unwrap()
    );
}
