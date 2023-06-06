use std::{env, process};

use line_counter::read_path_bfs;

fn main() {
    let args: Vec<String> = env::args().collect();

    // TODO – when no arguments provided, count lines in the directory from where executed
    if args.len() < 2 {
        println!("Please provide a path as a starting point");
        process::exit(1);
    }

    println!("{}", args[1]);
    let path = &args[1];

    read_path_bfs(path.to_string());
}
