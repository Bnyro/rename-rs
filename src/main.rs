use clap::{command, ArgAction, Parser};
use lazy_static::lazy_static;
use std::{
    fs::{self, metadata, rename},
    io,
    path::{Path, PathBuf},
    time::Instant,
};

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
    let now = Instant::now();
    let mut count = 0;

    let paths = if ARGS.recursively {
        list_files(&PathBuf::from(&ARGS.directory.clone())).expect("Unable to read directory")
    } else {
        fs::read_dir(ARGS.directory.clone())
            .unwrap()
            .filter_map(|path| path.ok())
            .map(|path| path.path())
            .collect()
    };

    let files = paths.iter().filter(|path| path.is_file());

    for file in files {
        if handle_file(&file) {
            count += 1
        }
    }

    let elapsed = now.elapsed();

    println!(">>> Modified {} files in {:.2?} <<<", count, elapsed);
}

fn _list_files(vec: &mut Vec<PathBuf>, path: &Path) -> io::Result<()> {
    if metadata(&path)?.is_dir() {
        let paths = fs::read_dir(&path)?;
        for path_result in paths {
            let full_path = path_result?.path();
            if metadata(&full_path)?.is_dir() {
                _list_files(vec, &full_path)?
            } else {
                vec.push(full_path);
            }
        }
    }
    Ok(())
}

fn list_files(path: &Path) -> io::Result<Vec<PathBuf>> {
    let mut vec = Vec::new();
    _list_files(&mut vec, &path)?;
    Ok(vec)
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
    let mut target = path.to_owned();
    target.set_file_name(&new_file_name);
    let _ = rename(path, target);

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
