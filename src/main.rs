#![allow(warnings)]

use std::{
    env::args,
    fs::{File, OpenOptions},
    io::{self, BufRead, BufReader, Seek, SeekFrom, Write},
    process::exit,
};

static PATHS_FILE: &str = "paths.txt";

#[derive(Debug)]
struct NavPath {
    name: String,
    path: String,
}

trait Finder {
    fn find(&self, target: &String) -> Option<&NavPath>;
}

impl Finder for Vec<NavPath> {
    fn find(&self, target: &String) -> Option<&NavPath> {
        self.iter().find(|e| e.name == target.as_str())
    }
}

fn main() -> io::Result<()> {
    let mut file: File = match OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(PATHS_FILE)
    {
        Ok(content) => content,
        Err(_) => {
            eprintln!("Failed to read file containg paths, the program will exit.");
            exit(1);
        }
    };
    // FIXME: Why do I need to clone this?
    let reader: BufReader<File> = BufReader::new(file.try_clone().unwrap());

    let mut paths: Vec<NavPath> = Vec::new();
    for line in reader.lines().flatten() {
        let parts: Vec<String> = line.split("::").map(String::from).collect();
        let n_path = NavPath {
            name: parts.get(0).unwrap().to_string(),
            path: parts.get(1).unwrap().to_string(),
        };
        paths.push(n_path);
    }

    let mut result = "";
    match args().count() {
        2 => {
            let arg: String = args().nth(1).unwrap();
            match arg.as_str() {
                "ls" => {
                    print_paths_short(&paths);
                }
                "ll" => {
                    print_paths_long(&paths);
                }
                _ => {
                    // TODO: try to navigate to the path under the name arg
                    if let Some(p) = paths.find(&arg) {
                        result = p.path.as_str();
                    } else {
                        eprintln!("Invalid arguments.");
                        print_usage();
                        exit(1);
                    }
                }
            }
        }
        3 => {
            let arg: String = args().nth(1).unwrap();
            match arg.as_str() {
                "clear" => clear_path(&mut file, &mut paths)?,
                _ => {
                    eprintln!("Invalid arguments.");
                    print_usage();
                    exit(1);
                }
            }
        }
        4 => {
            let arg: String = args().nth(1).unwrap();
            match arg.as_str() {
                "add" => add_path(&mut file, &mut paths)?,
                _ => {
                    eprintln!("Invalid arguments.");
                    print_usage();
                    exit(1);
                }
            }
        }
        _ => {
            eprintln!("Invalid number of arguments.");
            print_usage();
            exit(1);
        }
    }

    println!("{}", result);

    Ok(())
}

fn clear_path(file: &mut File, paths: &mut Vec<NavPath>) -> io::Result<()> {
    if let Some(target) = args().nth(2) {
        let mut new_content: String = String::new();

        for p in paths.iter().filter(|p| p.name != target) {
            new_content.push_str(format!("{}::{}\n", p.name, p.path).as_str());
        }

        file.seek(SeekFrom::Start(0));
        file.set_len(0);
        file.write_all(new_content.as_bytes());
    } else {
        print_usage();
    }
    Ok(())
}

fn add_path(file: &mut File, paths: &mut Vec<NavPath>) -> io::Result<()> {
    if let Some(name) = args().nth(2) {
        if let Some(path) = args().nth(3) {
            paths.push(NavPath { name, path });

            let mut new_content: String = String::new();
            for p in paths {
                new_content.push_str(format!("{}::{}\n", p.name, p.path).as_str());
            }

            file.seek(SeekFrom::Start(0));
            file.set_len(0);
            file.write_all(new_content.as_bytes());
        } else {
            print_usage();
        }
    } else {
        print_usage();
    }
    Ok(())
}

fn print_paths_short(paths: &Vec<NavPath>) -> () {
    for p in paths {
        print!("{} ", p.name);
    }
}

fn print_paths_long(paths: &Vec<NavPath>) -> () {
    for p in paths {
        println!("{} :: {}", p.name, p.path);
    }
}

fn print_usage() -> () {
    // TODO: implement usage printing
    println!("Usage: ...");
}
