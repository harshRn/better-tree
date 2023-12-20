use std::fs;
use colored::*;

fn print_dir_content(path: &str, ntabs: Option<usize>, curr_layer: Option<i32>, max_layer: Option<i32>) {
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(metadata) = entry.metadata() {
                    if let Some(entry_name) = entry.file_name().to_str() {
                        let spaces = " ".repeat(ntabs.unwrap_or(0));
                        if curr_layer == max_layer {
                            return ;
                        }
                        if metadata.is_dir() {
                            println!("{}{}", spaces, entry_name.blue());
                            print_dir_content(entry.
                                                    path().
                                                    to_str().
                                                    unwrap_or("-"),
                                                    Some(ntabs.unwrap_or(0) + 3),
                                                    Some(curr_layer.unwrap_or(0) + 1),
                                                    max_layer);
                        } else {
                            println!("{}{}", spaces, entry_name);
                        }
                    }
                }
            }
        }
    } else {
        println!("Failed to read the directory contents");
    }
}

fn main() {
    print_dir_content(".", Some(0), Some(0), Some(2));
}
