use clap::Parser;

#[derive(Parser)]
struct CLIArgs {
    pattern: String,
    path: std::path::PathBuf,
}

// #[derive(Debug)]
// struct CustomeError(String);

// impl std::fmt::Display for CustomeError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "CustomeError: {}", self.0)
//     }
// }

fn main() {
    // let pattern = std::env::args().nth(1).expect("no pattern given!");
    // let path = std::env::args().nth(2).expect("no path given!");
    //
    // let args = CLIArgs {
    //     pattern: pattern,
    //     path: std::path::PathBuf::from(path),
    // };
    //

    let args = CLIArgs::parse();

    // let's start by reading the file we got
    let content = std::fs::read_to_string(&args.path).unwrap();

    // let content = match result {
    //     Ok(content) => content,
    //     Err(e) => {
    //         panic!("Can't deal with error: '{}', just exit here!", e);
    //     }
    // };

    // display lines containing pattern
    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}
