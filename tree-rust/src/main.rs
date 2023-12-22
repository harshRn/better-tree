use colored::*;
use getopts::Options;
use std::{fs, usize};
extern crate getopts;

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
                        let sub_stats =
                            print_dir_content(&sub_path, ntabs + 3, curr_layer + 1, max_layer);
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
        .map(|s| s.parse::<i32>().unwrap_or(5))
        .unwrap_or(5);

    let stats = print_dir_content(root_path, 0, 0, max_layer);

    println!("\n{} directories, {} files", stats.dirs, stats.files);
}
