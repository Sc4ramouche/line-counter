use std::{collections::VecDeque, fs};

pub fn read_path_bfs(path: String) {
    let mut deq = VecDeque::from([path]);
    let mut file_count = 0;

    while !deq.is_empty() {
        let next_path = deq.pop_front().unwrap();

        // TODO: refactor to use ? operator to avoid crazy nesting
        match fs::read_dir(&next_path) {
            Ok(entries) => {
                for entry in entries {
                    if let Ok(entry) = entry {
                        if let Ok(file_type) = entry.file_type() {
                            if file_type.is_dir() {
                                let dir_name = entry.path().to_string_lossy().to_string();
                                deq.push_back(dir_name);
                            }

                            file_count += 1;
                        }
                    }
                }
            }
            Err(err) => eprintln!("Could not read path {} – {}", next_path, err),
        }
    }

    println!("File count: {}", file_count);
}
