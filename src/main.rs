use std::io::{self, Write};

mod shell;

fn main() {
    let mut buf = String::new();

    println!("demo how to do a Bash Clone");

    loop {
        print!("BASHCLONE# ");
        io::stdout().flush().unwrap();
        buf.clear();
        if io::stdin().read_line(&mut buf).is_err() {
            break;
        }

        shell::shell_parse(&buf);
    }
}
