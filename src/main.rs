#![allow(warnings)]

use std::{
    any::Any,
    env::{self, args},
    fs::{self, create_dir_all, File, OpenOptions},
    io::{self, BufRead, BufReader, Seek, SeekFrom, Write},
    process::exit,
};

use clap::{command, parser::ValuesRef, value_parser, Arg, ArgAction, ArgMatches, Args, Command};

#[derive(Debug)]
struct NavPath {
    name: String,
    path: String,
}

fn main() -> io::Result<()> {
    let mut command = parse_args();
    let matches = command.clone().get_matches();
    
    let mut paths: Vec<NavPath> = Vec::new();
    let mut file: File = open_config_file();

    load_paths(&mut paths, &file);

    /// ArgMatches::contains_id returns 'true' if default_value has been set
    /// ArgActions::SetTrue implies default_value 'false'
    if *matches.get_one::<bool>("list").unwrap() {
        print_paths(&paths);
    }

    if *matches.get_one::<bool>("purge").unwrap() {
        delete_all(&mut file);
    }

    if let Some(target) = matches.get_one::<String>("remove") {
        clear_path(&mut file, &mut paths, target);
    }

    if matches.contains_id("add") {
        let values: Vec<String> = matches
            .get_many("add")
            .expect("expect the unexpected")
            .cloned()
            .collect();

        let name: &String = values.get(0).unwrap();
        let path: &String = values.get(1).unwrap();

        add_path(&mut file, &mut paths, name, path);
    }

    Ok(())
}

fn parse_args() -> Command {
    command!()
        .arg_required_else_help(true)
        .arg(
            Arg::new("list options")
                .id("list")
                .short('l')
                .long("list")
                .help("Print available navigator paths")
                .required(false)
                .action(ArgAction::SetTrue)
                .conflicts_with_all(&["purge", "add", "remove"]),
        )
        .arg(
            Arg::new("purge")
                .id("purge")
                .long("purge")
                .help("Delete all navigator paths")
                .required(false)
                .action(ArgAction::SetTrue)
                .conflicts_with_all(&["list", "add", "remove"]),
        )
        .arg(
            Arg::new("add item")
                .id("add")
                .short('a')
                .long("add")
                .help("Add new navigator item")
                .action(ArgAction::Append)
                .value_parser(value_parser!(String))
                .num_args(2)
                .value_names(&["item_name", "item_path"])
                .conflicts_with_all(&["list", "purge", "remove"]),
        )
        .arg(
            Arg::new("Remove item")
                .id("remove")
                .short('r')
                .long("remove")
                .help("Delete navigator item by name")
                .required(false)
                .value_name("name")
                .conflicts_with_all(&["list", "purge", "add"]),
        )
}

fn load_paths(paths: &mut Vec<NavPath>, file: &File) {
    let reader: BufReader<File> = BufReader::new(file.try_clone().unwrap());
    for line in reader.lines().flatten() {
        let parts: Vec<String> = line.split("::").map(String::from).collect();
        let n_path = NavPath {
            name: parts.get(0).unwrap().to_string(),
            path: parts.get(1).unwrap().to_string(),
        };
        paths.push(n_path);
    }
}

fn open_config_file() -> File {
    let config_file_name: String = String::from("navi_gator.cfg");
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

    let path: String = format!("{}/{}", directory_path, config_file_name);

    match OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
    {
        Ok(content) => content,
        Err(_) => {
            eprintln!("Failed to read file containg paths, the program will exit.");
            exit(0)
        }
    }
}

/// Delete all the navigation paths
/// This also deletes them from the config file
fn delete_all(file: &mut File) -> () {
    file.set_len(0);
}

/// Deletes the single navigator path which matches the name under 'target'
fn clear_path(file: &mut File, paths: &mut Vec<NavPath>, target: &String) -> io::Result<()> {
    let mut new_content: String = String::new();

    for p in paths.iter().filter(|p| p.name != *target) {
        new_content.push_str(format!("{}::{}\n", p.name, p.path).as_str());
    }

    file.seek(SeekFrom::Start(0));
    file.set_len(0);
    file.write_all(new_content.as_bytes());

    println!("Successfully removed {}", target);
    Ok(())
}

fn add_path(
    file: &mut File,
    paths: &mut Vec<NavPath>,
    name: &String,
    path: &String,
) -> io::Result<()> {
    if let Some(existing) = paths.iter().find(|p| p.name == *name) {
        println!(
            "A navigator path with tha name: '{}' already exists for the path: '{}'",
            existing.name, existing.path
        );
        exit(0);
    }

    let path: String = match path.as_str() {
        "." |
        "./" => env::current_dir().unwrap().to_str().unwrap().to_string(),
        _ => path.clone(),
    };

    if let Err(e) = fs::metadata(&path) {
        eprintln!("The path provided: '{}' doesn't exist.", path);
        // print_usage();
        exit(1);
    }

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
    Ok(())
}

fn print_paths(paths: &Vec<NavPath>) -> () {
    if paths.is_empty() {
        println!("You have no navigation paths yet.");
    }

    for p in paths {
        eprintln!("{:10} :: {}", p.name, p.path);
    }
}
