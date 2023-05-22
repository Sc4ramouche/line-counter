use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide a path as a starting point");
        process::exit(1);
        // TODO – when no arguments provided, count lines in the directory from where executed
    }

    println!("{}", args[1]);
    let path = &args[1];

    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let file_name = entry.file_name();
                    println!("File name: {}", file_name.to_string_lossy());

                    if let Ok(file_type) = entry.file_type() {
                        println!("File type: {:?}\n", file_type);
                    }
                }
            }
        }
        Err(err) => eprintln!("Could not read path {} – {}", path, err),
    }
}
