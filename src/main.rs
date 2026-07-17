use std::io;
use std::io::Write;

fn main() {
    // for test
    run_loop();
}

fn run_loop() {
    let mut flag = true;
    let mut buf = String::new();

    while flag {
        print!("$ ");
        io::stdout().flush().expect("failed to flush stdout");
        buf.clear();
        io::stdin().read_line(&mut buf).expect("failed to read line");
        if buf.trim() != "exit" {
            print!("{}", buf);
        } else {
            flag = false;
        }
    }
}
