use std::{fs, path::PathBuf};

pub fn change_dir(program_path: &mut PathBuf, path: &str) {
    String::from(path)
        .replace("//", "/") // clean "//"
        .replace("\\", "/") // clean "\\"
        .split('/')
        .for_each(|value| {
            if value.starts_with("..") {
                program_path.pop();
            } else if value.starts_with(".") || value == "" {
            } else {
                program_path.push(value.to_owned());

                // NOT exists file
                if !program_path.exists() {
                    program_path.pop();
                    println!(">> Failed to resolve this path");
                }
            }
        });
}

pub fn list_dir(program_path: &PathBuf) {
    for i in fs::read_dir(&program_path).expect(">> Failed to read dir.") {
        println!("{:?}", i.unwrap().file_name());
    }
}

pub fn make_dir(program_path: &PathBuf, paths: &[String]) {
    for path in paths {
        let path = program_path.join(path);
        
        if let Err(err) = fs::create_dir_all(path) {
            eprintln!(">> Failed to make dir: {}", err);
        }
    }
}

pub fn touch(program_path: &PathBuf, paths: &[String]) {
    for path in paths {
        let path = program_path.join(path);

        if let Err(err) = fs::File::create(path) {
            eprintln!(">> Failed to create file: {}", err);
        }
    }
}

pub fn remove_dir(path: &PathBuf) {
    if let Err(err) = fs::remove_dir_all(path) {
        eprintln!(">> Failed to remove dir: {}", err);
    }
}

pub fn remove_file(path: &PathBuf) {
    if let Err(err) = fs::remove_file(path) {
        eprintln!(">> Failed to remove file: {}", err);
    }
}

pub fn remove(program_path: &PathBuf, paths: &[String]) {
    for path in paths {
        let path = program_path.join(path);

        if path.is_dir() {
            remove_dir(&path);
        } else {
            remove_file(&path);
        }
    }
}
