use std::fs::DirEntry;
use std::process::Command;
use std::{env, fs};
use std::path::PathBuf;


pub fn build() {
    let mut dir = env::current_dir().expect("Invalid directory (This directory ether does not exist or we don't have permission)");

    //todo change to src
    dir.push("test");
    compile_dir(dir);

    link();
}

fn compile_dir(path: PathBuf) {
    let current_dir = env::current_dir().expect("Invalid directory (This directory ether does not exist or we don't have permission)");

    let paths = fs::read_dir(path).unwrap();
    for path in paths {
        let path = path.unwrap();
        if path.file_type().unwrap().is_file() {
            let path_buf = path.path();
            let file_type = path_buf.extension().unwrap();

            //todo add other c++ files
            if file_type == "cpp" || file_type == "tpp" {
                compile(path);
            }
        } else {
            compile_dir(path.path());
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
        //todo change binT to bin
        .arg("-o").arg(format!("binT/{}.o", file_name))
        .output()
        //todo better compile error message
        .expect("Failure to compile");
}

fn link() {
    let mut bin = env::current_dir().unwrap();
    //todo change binT to bin
    bin.push("binT");
    let files = fs::read_dir(bin).unwrap();

    let files = files.into_iter().map(|dir_entry| {
        let dir_entry = dir_entry.unwrap();
        let path = dir_entry.path();
        path.to_str().unwrap().to_owned()
    }).collect::<Vec<_>>();

    let files = extract_o_files(files);

    let output = Command::new("g++")
        .args(files)
        //todo change binT to bin
        //todo change output to whatever the project name is
        .arg("-o").arg("binT/output.exe")
        .output()
        //todo better link error message
        .expect("Failure to link"); 
}

fn extract_o_files(files: Vec<String>) -> Vec<String> {
    let mut ok = Vec::new();

    for file in files {
        let spot = file.rfind(".");
        if spot.is_none() {
            continue;
        }
        let spot = spot.unwrap();

        let extension = &file[spot..];

        if extension == ".o" {
            ok.push(file);
        }
    }

    ok

}

