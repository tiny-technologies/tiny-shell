use std::{
    env,
    io::{self, Write},
    path::Path,
};

fn main() {
    loop {
        let current_dir = env::current_dir().unwrap();
        print!(
            "\x1b[1;31mtinyshell\x1b[0m 🤏 in \x1b[1;34m{}\x1b[0m\n$ ",
            current_dir.display()
        );
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).unwrap() == 0 {
            println!();
            return;
        }

        let mut args = input.trim().split_ascii_whitespace();
        let command = match args.next() {
            Some(command) => command,
            None => continue,
        };

        match command {
            "" => continue,
            "cd" => {
                let new_dir = args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);
                if let Err(error) = env::set_current_dir(&root) {
                    eprintln!("{}", error);
                }
            }
            "c" => print!("\x1b[2J\x1b[1;1H"),
            "exit" => return,
            command => {
                match std::process::Command::new(command).args(args).spawn() {
                    Ok(mut child) => match child.wait().unwrap().code() {
                        Some(0) => {}
                        Some(code) => println!("Child exited with status code: {}", code),
                        None => println!("Process terminated by signal"),
                    },
                    Err(error) => match error.kind() {
                        io::ErrorKind::NotFound => {
                            eprintln!("myshell: {} command not found!", command)
                        }
                        _ => eprintln!("{}", error),
                    },
                };
            }
        }
    }
}
