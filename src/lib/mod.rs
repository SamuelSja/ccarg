pub mod compile;


use std::fs::DirEntry;
use std::process::Command;
use std::{env, fs};
use std::path::PathBuf;


pub fn start() {
    let op = Operation::parse();
    process_op(op);    
}

pub fn run() {
    let output = Command::new("binT/output")
        .output()
        .unwrap();

    let stdout = output.stdout;
    let stdout = String::from_utf8(stdout).expect("cannot print output");

    print!("{}", stdout);
    
}

fn process_op(op: Operation) {
    match op {
        Operation::Run => {
            compile::build();
            run();
        },
        Operation::Build => {
            compile::build();
        },
        Operation::New {name} => {
            unimplemented!();
        },
        _ => panic!("unknown command"),
    } 
}


#[non_exhaustive]
pub enum Operation {
    Run,
    Build,
    New {name: String},
}

impl Operation {
    pub fn parse() -> Self {
        let command = env::args().nth(1).expect("invalid call");
        if command == "run" || command == "r" {
            return Self::Run;
        } else if command == "build" || command == "b" {
            return Self::Build;
        } else if command == "new" || command == "n" {
            return Self::New {
                name: env::args().nth(2).expect("No name found"),
            }
        }

        panic!("invalid call");
    }
}
