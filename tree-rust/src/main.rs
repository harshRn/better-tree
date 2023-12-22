use colored::*;
use std::fs;

struct Stats {
    dirs: usize,
    files: usize,
}

fn print_dir_content(path: &str, ntabs: usize, curr_layer: i32, max_layer: i32) -> Stats {
    let mut dir_stats = Stats { dirs: 0, files: 0 };

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.filter_map(Result::ok) {
            if let Ok(metadata) = entry.metadata() {
                if let Some(entry_name) = entry.file_name().to_str() {
                    if entry_name.starts_with(".") {
                        continue;
                    }
                    let spaces = " ".repeat(ntabs);
                    if curr_layer == max_layer {
                        return dir_stats;
                    }
                    if metadata.is_dir() {
                        dir_stats.dirs += 1;
                        println!("{}{}", spaces, entry_name.blue());
                        let sub_path = entry.path().to_string_lossy().into_owned();
                        let sub_stats = print_dir_content(&sub_path, ntabs + 3, curr_layer + 1, max_layer);
                        dir_stats.dirs += sub_stats.dirs;
                        dir_stats.files += sub_stats.files;
                    } else {
                        dir_stats.files += 1;
                        println!("{}{}", spaces, entry_name);
                    }
                }
            }
        }
    }

    dir_stats
}

fn main() {
    let root_path = ".";
    let stats = print_dir_content(root_path, 0, 0, 3);

    println!("\n{} directories, {} files", stats.dirs, stats.files);
}
