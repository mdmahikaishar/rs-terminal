mod command;
mod utils;

use command::Command;
use std::io::{self, Write};
use std::path::PathBuf;

fn main() {
    let mut program_path = PathBuf::from("C:/Users");
    let mut command = Command::new();

    println!(">> RS TERMINAL");

    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        //  take input
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect(">> failed to take input.");

        // command
        command.prase(input);

        // "cd", "ls", "touch", "mkdir", "rm", "cwd()"
        match &command.name[..] {
            "-h" | "--help" => {
                println!("Usage:
                \r\ncd   : change working directory.
                \r\nls   : list of all files and directories.
                \r\ntouch: make files.
                \r\nmkdir: make directories.
                \r\nrm   : remove files and directories.
                \r\nexit : exit the program.
                    ");
            },
            "ls" => utils::list_dir(&program_path),
            "cd" => utils::change_dir(&mut program_path, &command.values[0]),
            "touch" => utils::touch(&program_path, &command.values),
            "mkdir" => utils::make_dir(&program_path, &command.values),
            "rm" => utils::remove(&program_path, &command.values),
            "exit" => break,
            _ => {
                println!(">> Invalid command {}", command.name)
            }
        }
    }
}
