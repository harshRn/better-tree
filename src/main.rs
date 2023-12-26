use colored::*;
use getopts::Options;
use std::{fs, usize};
extern crate getopts;

struct Stats {
    dirs: usize,
    files: usize,
}

fn print_entry(prefix: &str, entry_name: &str, is_last: bool, color: Color) {
    let mark = if is_last { "└── " } else { "├── " };
    println!("{}{}{}", prefix, mark, entry_name.color(color));
}

fn print_dir_content(path: &str, prefix: &str, curr_layer: i32, max_layer: i32) -> Stats {
    let mut dir_stats = Stats { dirs: 0, files: 0 };

    if let Ok(entries) = fs::read_dir(path) {
        let mut entries: Vec<_> = entries.collect::<Result<_, _>>().unwrap_or_else(|_| vec![]);
        entries.sort_by(|a, b| {
            let a_file_name = a.file_name();
            let b_file_name = b.file_name();

            let a_name = a_file_name.to_string_lossy();
            let b_name = b_file_name.to_string_lossy();

            let a_starts_with_dot = a_name.starts_with(".");
            let b_starts_with_dot = b_name.starts_with(".");

            if a_starts_with_dot && !b_starts_with_dot {
                std::cmp::Ordering::Less
            } else if !a_starts_with_dot && b_starts_with_dot {
                std::cmp::Ordering::Greater
            } else {
                a_name
                    .partial_cmp(&b_name)
                    .unwrap_or(std::cmp::Ordering::Equal)
            }
        });
        let total_entries = entries.len();
        for (index, entry) in entries.iter().enumerate() {
            if let Ok(metadata) = entry.metadata() {
                if let Some(entry_name) = entry.file_name().to_str() {
                    if entry_name.starts_with(".") {
                        continue;
                    }
                    if curr_layer == max_layer {
                        return dir_stats;
                    }
                    let is_last = index == total_entries - 1;
                    let mark = if is_last { " " } else { "│" };
                    if metadata.is_dir() {
                        dir_stats.dirs += 1;
                        print_entry(prefix, entry_name, is_last, Color::Blue);
                        let sub_path = entry.path().to_string_lossy().into_owned();
                        let sub_stats = print_dir_content(
                            &sub_path,
                            &format!("{}{}   ", prefix, mark),
                            curr_layer + 1,
                            max_layer,
                        );
                        dir_stats.dirs += sub_stats.dirs;
                        dir_stats.files += sub_stats.files;
                    } else {
                        dir_stats.files += 1;
                        print_entry(prefix, entry_name, is_last, Color::BrightWhite);
                    }
                }
            }
        }
    }

    dir_stats
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program.split("/").last().unwrap());
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let program = args[0].clone();
    let mut opts = getopts::Options::new();

    opts.optopt(
        "d",
        "depth",
        "limits the depth of the directory scaning",
        "[1-9]",
    );
    opts.optflag("h", "help", "show this information");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Error: {}", e);
            print_usage(&program, opts);
            return;
        }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    let root_path: &str;
    if let Some(p) = matches.free.get(0) {
        root_path = &p;
    } else {
        root_path = ".";
    }
    let max_layer: i32 = matches
        .opt_str("d")
        .map(|s| s.parse::<i32>().unwrap_or(3))
        .unwrap_or(3);

    println!(".");
    let stats = print_dir_content(root_path, "", 0, max_layer);

    println!("\n{} directories, {} files", stats.dirs, stats.files);
}
