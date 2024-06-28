use std::fs::DirEntry;
use std::process::Command;
use std::{env, fs};
use std::path::PathBuf;


pub mod lib;
fn main() {
    // let op = Operation::parse();
    let mut dir = env::current_dir().expect("Invalid directory (This directory ether does not exist or we don't have permission)");
    dir.push("test");
    build_in(dir);
}

fn process_op(op: Operation) {
    match op {
        Operation::Run => {

        },
        Operation::Build => {

        },
        Operation::New {name} => {

        },
        _ => panic!("unknown command"),
    } 
}

fn build() {
    let current_dir = env::current_dir().expect("Invalid directory (This directory ether does not exist or we don't have permission)");
}

fn build_in(path: PathBuf) {
    let current_dir = env::current_dir().expect("Invalid directory (This directory ether does not exist or we don't have permission)");

    let paths = fs::read_dir(path).unwrap();
    for path in paths {
        let path = path.unwrap();
        if path.file_type().unwrap().is_file() {
            compile(path);
        } else {
            build_in(path.path());
        }
    }
}

fn compile(path: DirEntry) {
    let file_name = {
        let name = path.file_name()
            .into_string()
            .unwrap();
        let spot = name.rfind(".").unwrap();
        name[0..spot].to_owned()
    };

    let output = Command::new("g++")
        .arg(path.path())
        .arg("-c")
        .arg("-o")
        .arg(format!("binT/{}.o", file_name))
        .output()
        .expect("Failure to compile");

    println!("compiled, output: {output:?}");
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