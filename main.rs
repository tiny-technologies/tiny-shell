use std::{
    env,
    io::{self, stdin, stdout, Write},
    path::Path,
};

fn main() {
    loop {
        print!(
            "\x1b[1;31mtiny-shell\x1b[0m 🤏 in \x1b[1;34m{}\x1b[0m\n$ ",
            env::current_dir().unwrap().display()
        );
        stdout().flush().unwrap();

        let mut input = String::new();
        if stdin().read_line(&mut input).unwrap() == 0 {
            println!();
            return;
        }

        let mut args = input.trim().split_ascii_whitespace();
        let command = match args.next() {
            Some(command) => command,
            None => continue,
        };

        match command {
            "help" => println!("The following commands are available: help, cd, clear, exit"),
            "cd" => {
                // todo revisse
                let new_dir = args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);
                if let Err(error) = env::set_current_dir(&root) {
                    eprintln!("{}", error);
                }
            }
            "clear" => print!("\x1b[2J\x1b[1;1H"),
            "exit" => return,
            command => {
                match std::process::Command::new(command).args(args).spawn() {
                    Ok(mut child) => match child.wait().unwrap().code() {
                        Some(0) => println!("✅"),
                        Some(code) => println!("❌ Child exited with status code: {}", code),
                        None => println!("Process terminated by signal"),
                    },
                    Err(error) => match error.kind() {
                        io::ErrorKind::NotFound => {
                            eprintln!("tiny-shell: {} command not found!", command)
                        }
                        _ => eprintln!("{}", error),
                    },
                };
            }
        }
    }
}
