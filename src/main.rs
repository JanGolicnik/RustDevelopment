use std::{
    env, fs,
    path::Path,
    process::{Command, Output},
};

use parsing::parse;
use tokenization::{tokenize, Token};

mod parse_error;
mod parsing;
mod tokenization;
mod tokenization_error;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("please provide a filename");
        return;
    }

    let file_path = Path::new(&args[1]);
    let file_string = fs::read_to_string(file_path).expect("Unable to read file!");

    match tokenize(&file_string) {
        Ok(tokens) => {
            tokens.iter().for_each(|token| println!("{:?}", token));
            match parse(tokens) {
                Ok(output) => assemble(output),
                Err(e) => eprintln!("{}", e),
            }
        }
        Err(e) => eprintln!("{}", e),
    }
}

fn assemble(code: String) {
    println!("OUTPUT: \n{code}");

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

fn tokens_to_assembly(tokens: Vec<Token>) -> String {
    let mut output: String = "global _start\n_start:\n".to_string();

    let mut tokens_iter = tokens.iter().enumerate();

    while let Some((index, token)) = tokens_iter.next() {
        match token {
            Token::Return => {
                if index + 2 < tokens.len() {
                    match tokens.get(index + 1) {
                        Some(Token::Int(int_val)) => match tokens.get(index + 2) {
                            Some(Token::Semicolon) => {
                                output += "    mov rax, 60\n";
                                output += format!("    mov rdi, {}\n", int_val).as_str();
                                output += "    syscall\n";

                                tokens_iter.nth(1);
                                continue;
                            }
                            _ => {
                                eprintln!("EXPECTED Semicolon fter int AFTER RETURN");
                                return "".to_string();
                            }
                        },
                        _ => {
                            eprintln!("EXPECTED INT LITERAL AFTER RETURN");
                            return "".to_string();
                        }
                    }
                }
            }
            _ => {
                eprintln!("UNEXPECTED TOKEN");
                return "".to_string();
            }
        }
    }

    output
}
