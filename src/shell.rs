use std::io;

fn echo_main(args: Vec<&str>) -> io::Result<()> {
    if args.len() > 1 {
        println!("{}", args[1]);
    }
    Ok(())
}

struct Operation {
    name: &'static str,
    func: fn(Vec<&str>) -> io::Result<()>,
}

static OPS: &[Operation] = &[
    Operation { name: "echo", func: echo_main },
];

fn command_do(args: Vec<&str>) -> io::Result<()> {
    for op in OPS.iter() {
        if args.first() == Some(&op.name) {
            (op.func)(args)?;
            break;
        }
    }
    Ok(())
}

pub fn shell_parse(input: &str) -> io::Result<()> {
    let mut args = vec![];
    let mut current_arg = String::new();
    let mut in_word = false;

    for c in input.chars() {
        match c {
            ' ' | '\n' if in_word => {
                args.push(current_arg.trim_end());
                current_arg.clear();
                in_word = false;
            },
            ' ' | '\n' => (),
            _ => {
                current_arg.push(c);
                in_word = true;
            }
        }
    }
    if !current_arg.is_empty() {
        args.push(current_arg.trim_end());
    }

    command_do(args)
}
