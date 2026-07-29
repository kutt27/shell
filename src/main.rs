use std::io;
use std::io::Write;

fn main() {
    // for test
    run_loop();
}

fn run_loop() {
    let mut buf = String::new();
    let allowed_commands: [&str; 1] = ["hello"];
    let mut exit = false;

    while !exit {
        print!("$ ");
        io::stdout().flush().expect("failed to flush stdout");

        buf.clear();
        io::stdin().read_line(&mut buf).expect("failed to read line");
        
        let trimmed = buf.trim();
        if trimmed.is_empty() {
            continue; // ignore empty lines
        }

        let args: Vec<&str> = buf.trim().split_whitespace().collect();
        
        if allowed_commands.contains(&args[0]) {
            print!("{}", buf);
        } else if args[0] == "exit" {
            println!("exited with {}", 0);
            exit = true;
        } else {
            println!("{}: command not found", args[0]);
        }
    }
}
