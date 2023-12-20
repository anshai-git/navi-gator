#![allow(warnings)]

use std::{
    env::{self, args},
    fs::{self, create_dir_all, File, OpenOptions},
    io::{self, BufRead, BufReader, Seek, SeekFrom, Write},
    process::exit,
};

trait Finder {
    fn find(&self, target: &String) -> Option<&NavPath>;
}

#[derive(Debug)]
struct NavPath {
    name: String,
    path: String,
}

impl Finder for Vec<NavPath> {
    fn find(&self, target: &String) -> Option<&NavPath> {
        self.iter().find(|e| e.name == target.as_str())
    }
}

#[derive(Debug)]
enum Action {
    LL,
    ADD,
    CLEAN,
    REMOVE,
    HELP,
}

impl Action {
    const VALUES: [Self; 5] = [Self::LL, Self::ADD, Self::REMOVE, Self::CLEAN, Self::HELP];

    fn name(&self) -> String {
        match self {
            Action::LL => String::from("ll"),
            Action::ADD => String::from("add"),
            Action::CLEAN => String::from("clean"),
            Action::REMOVE => String::from("remove"),
            Action::HELP => String::from("help"),
        }
    }

    fn description(&self) -> String {
        match self {
            Action::LL => String::from("List the available arguments with more info."),
            Action::ADD => String::from("Add new navigator item."),
            Action::CLEAN => String::from("Delete all navigation items"),
            Action::REMOVE => String::from("Delete navigator item."),
            Action::HELP => String::from("Print usage."), }
    }

    fn from_name(target: &String) -> Option<Action> {
        match target.as_str() {
            "ll" => Some(Action::LL),
            "add" => Some(Action::ADD),
            "clean" => Some(Action::CLEAN),
            "remove" => Some(Action::REMOVE),
            "help" => Some(Action::HELP),
            _ => None,
        }
    }
}

fn main() -> io::Result<()> {
    let config_file_name: String = String::from("config.txt");
    let directory_path = match env::var("HOME") {
        Ok(home_path) => format!("{}/.config/navi-gator/", home_path),
        Err(_) => {
            eprintln!("Failed to read home directory.");
            exit(0)
        }
    };

    if let Err(e) = create_dir_all(&directory_path) {
        eprintln!("Error creating config directory: {}", e);
    }

    let config_path: String = format!("{}/{}", directory_path, config_file_name);
    let mut file: File = match OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(config_path)
    {
        Ok(content) => content,
        Err(_) => {
            eprintln!("Failed to read file containg paths, the program will exit.");
            exit(0)
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

    if args().count() > 1 {
        let arg: String = args().nth(1).unwrap();
        if let Some(ca) = Action::from_name(&arg) {
            match ca {
                Action::LL => {
                    print_paths(&paths);
                }
                Action::ADD => {
                    add_path(&mut file, &mut paths);
                }
                Action::CLEAN => {
                    delete_all(&mut file);
                }
                Action::REMOVE => {
                    clear_path(&mut file, &mut paths);
                }
                Action::HELP => {
                    print_usage();
                }
            };
            exit(0)
        }

        if let Some(to_path) = paths.find(&arg) {
            println!("CHANGE_DIR {}", to_path.path.as_str());
            exit(0)
        } else {
            eprintln!("Invalid arguments.");
            print_usage();
            exit(0)
        }
    } else {
        eprintln!("Invalid arguments.");
        print_usage();
    }

    Ok(())
}

fn delete_all(file: &mut File) -> () {
    file.set_len(0);
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

        println!("Successfully removed {}", target);
    } else {
        print_usage();
    }
    Ok(())
}

fn add_path(file: &mut File, paths: &mut Vec<NavPath>) -> io::Result<()> {
    if let Some(name_arg) = args().nth(2) {
        if let Some(existing) = paths.iter().find(|p| p.name == name_arg) {
            println!(
                "A navigator path with tha name: '{}' already exists for the path: '{}'",
                existing.name, existing.path
            );
            exit(0);
        }

        if let Some(path_arg) = args().nth(3) {
            if let Err(e) = fs::metadata(&path_arg) {
                eprintln!("The path provided: '{}' doesn't exist.", path_arg);
                print_usage();
                exit(1);
            }

            let name = name_arg;
            let path: String = match path_arg.as_str() {
                "." => env::current_dir().unwrap().to_str().unwrap().to_string(),
                _ => path_arg,
            };

            paths.push(NavPath {
                name: name.clone(),
                path: path.clone(),
            });
            let mut new_content: String = paths
                .iter()
                .map(|p| format!("{}::{}\n", p.name, p.path))
                .collect::<Vec<_>>()
                .join("");

            file.seek(SeekFrom::Start(0));
            file.set_len(0);
            file.write_all(new_content.as_bytes());

            println!("Successfully added {} pointing to: {}", name, path);
        } else {
            print_usage();
        }
    } else {
        print_usage();
    }
    Ok(())
}

fn print_paths(paths: &Vec<NavPath>) -> () {
    if paths.is_empty() {
        println!("You have no navigation paths yet.");
        print_usage();
    }

    for p in paths {
        eprintln!("{:10} :: {}", p.name, p.path);
    }
}

fn print_usage() -> () {
    eprintln!("Usage:");
    for a in Action::VALUES {
        eprintln!("{:10} - {}", a.name(), a.description());
    }
}
