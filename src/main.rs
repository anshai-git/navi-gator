use std::{
    env::args,
    fs::{File, OpenOptions},
    io::{self, BufRead, BufReader, Write},
    process::exit,
};

static PATHS_FILE: &str = "paths.txt";

#[derive(Debug)]
struct NavPath {
    name: String,
    path: String,
}

fn main() -> io::Result<()> {
    let mut file: File = match OpenOptions::new()
        .create(true)
        .write(true)
        .read(true)
        .append(true)
        .open(PATHS_FILE)
    {
        Ok(content) => content,
        Err(_) => {
            eprintln!("Failed to read file containg paths, the program will exit.");
            exit(1);
        }
    };
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

    println!("ARGS COUNT: {}", args().count());

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
                    eprintln!("Invalid arguments.");
                    print_usage();
                    exit(1);
                }
            }
        }
        4 => {
            let arg: String = args().nth(1).unwrap();
            match arg.as_str() {
                "add" => add_path(&mut file)?,
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

    println!("{:?}", paths);
    println!("This is the output");

    Ok(())
}

fn add_path(file: &mut File) -> io::Result<()> {
    if let Some(name) = args().nth(2) {
        if let Some(path) = args().nth(3) {
            write!(file, "{}::{}", name, path)?;
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
        println!("{}  ::  {}", p.name, p.path);
    }
}

fn print_usage() -> () {
    // TODO: implement usage printing
    println!("Usage: ...");
}
