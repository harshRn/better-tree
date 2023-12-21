use colored::*;
use std::fs;

fn print_dir_content(
    path: &str,
    ntabs: Option<usize>,
    curr_layer: Option<i32>,
    max_layer: Option<i32>,
) -> Option<(i32, i32)> {
    let mut ndirs = 0;
    let mut nfiles = 0;

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(metadata) = entry.metadata() {
                    if let Some(entry_name) = entry.file_name().to_str() {
                        let spaces = " ".repeat(ntabs.unwrap_or(0));
                        if curr_layer == max_layer {
                            return Some((ndirs, nfiles));
                        }
                        if metadata.is_dir() {
                            ndirs += 1;
                            println!("{}{}", spaces, entry_name.blue());
                            if let Some((sub_ndirs, sub_nfiles)) = print_dir_content(
                                entry.path().to_str().unwrap_or(""),
                                Some(ntabs.unwrap_or(0) + 3),
                                Some(curr_layer.unwrap_or(0) + 1),
                                max_layer,
                            ) {
                                ndirs += sub_ndirs;
                                nfiles += sub_nfiles;
                            }
                        } else {
                            nfiles += 1;
                            println!("{}{}", spaces, entry_name);
                        }
                    }
                }
            }
        }
        return Some((ndirs, nfiles));
    } else {
        None
    }
}

fn main() {
    if let Some((total_dirs, total_files)) = print_dir_content(
        ".",
        Some(0),
        Some(0),
        Some(2)
    ) {
        println!("\nTotal dirs: {}\nTotal files: {}", total_dirs, total_files);
    }
}
