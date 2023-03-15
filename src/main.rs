use clap::{command, ArgAction, Parser};
use lazy_static::lazy_static;
use std::{
    fs::{self, rename},
    path::PathBuf,
};
use walkdir::WalkDir;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, action = ArgAction::SetTrue)]
    recursively: bool,
    #[arg(short, long, default_value = ".")]
    directory: PathBuf,
    #[arg()]
    from: String,
    #[arg()]
    to: String,
}

lazy_static! {
    static ref ARGS: Cli = Cli::parse();
}

fn main() {
    let mut count = 0;

    if ARGS.recursively {
        for file in WalkDir::new(ARGS.directory.clone())
            .into_iter()
            .filter_map(|file| file.ok())
        {
            if file.path().is_file() && handle_file(&file.path().to_path_buf()) {
                count += 1
            }
        }
    } else {
        let paths = fs::read_dir(ARGS.directory.clone()).unwrap();
        for path in paths {
            match path {
                Ok(path) => {
                    if path.path().is_file() && handle_file(&path.path()) {
                        count += 1
                    }
                }
                Err(_) => {}
            }
        }
    }

    println!(">>> Modified {} files", count);
}

fn handle_file(path: &PathBuf) -> bool {
    let file_name = path
        .file_name()
        .expect("File name not found")
        .to_string_lossy();

    let new_name = get_new_name(file_name.to_string());

    if new_name.is_none() {
        return false;
    }

    let new_file_name = new_name.unwrap().to_string();
    let _ = rename(path, &new_file_name);

    println!("Renamed {} to {}", file_name, new_file_name);

    return true;
}

fn get_new_name(file_name: String) -> Option<String> {
    let patterns = ARGS.from.split("*").filter(|pattern| !pattern.is_empty());
    let mut variables: Vec<&str> = vec![];
    let mut current_index: i32 = -1;

    for pattern in patterns {
        let occurences: Vec<_> = file_name
            .match_indices(pattern)
            .map(|(index, _)| index)
            .filter(|index| current_index < (*index as i32))
            .collect();
        match occurences.first() {
            Some(index) => {
                let old_index: usize = if current_index >= 0 {
                    current_index as usize
                } else {
                    0
                };
                let variable: &str = &file_name[old_index..*index];
                variables.push(&variable);
                let new_index = index + pattern.len();
                current_index = new_index as i32;
            }
            None => {
                return None;
            }
        }
    }

    let mut new_file_name = ARGS.to.clone();
    variables.iter().enumerate().for_each(|(i, var)| {
        new_file_name = new_file_name.replace(format!("${}", i + 1).as_str(), var);
    });

    return Some(new_file_name);
}
