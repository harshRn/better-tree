use crate::args::Settings;
use clap::Parser;
use colored::*;
use std::{
	fs::{self, DirEntry},
	usize,
};

mod args;

struct Stats {
	dirs: usize,
	files: usize,
}

fn print_entry(prefix: &str, entry_name: String, is_last: bool, color: Color) {
	let mark = if is_last { "└── " } else { "├── " };
	println!("{}{}{}", prefix, mark, entry_name.color(color));
}

fn process_entry(
	prefix: &str,
	entry: &DirEntry,
	is_last: bool,
	curr_depth: i32,
	settings: &Settings,
	dir_stats: &mut Stats,
) {
	let mark = if is_last { " " } else { "│" };
	if let Ok(meta) = entry.metadata() {
		if meta.is_dir() {
			dir_stats.dirs += 1;
			print_entry(
				prefix,
				entry
					.file_name()
					.into_string()
					.unwrap_or_else(|s| s.to_string_lossy().into_owned()),
				is_last,
				Color::Blue,
			);
			let sub_path = entry.path().to_string_lossy().into_owned();
			let sub_stats = print_dir_content(
				sub_path,
				&format!("{}{}   ", prefix, mark),
				curr_depth + 1,
				settings,
			);
			dir_stats.dirs += sub_stats.dirs;
			dir_stats.files += sub_stats.files;
		} else {
			dir_stats.files += 1;
			print_entry(
				prefix,
				entry
					.file_name()
					.into_string()
					.unwrap_or_else(|s| s.to_string_lossy().into_owned()),
				is_last,
				Color::BrightWhite,
			);
		}
	}
}

fn sort_entries(entries: &mut Vec<DirEntry>) {
	return entries.sort_by(|a, b| {
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
			let primary_order = a_name
				.partial_cmp(&b_name)
				.unwrap_or(std::cmp::Ordering::Equal);
			if primary_order == std::cmp::Ordering::Equal {
				a_name.cmp(&b_name)
			} else {
				primary_order
			}
		}
	});
}

fn print_dir_content(path: String, prefix: &str, curr_depth: i32, settings: &Settings) -> Stats {
	let mut dir_stats = Stats { dirs: 0, files: 0 };

	if let Ok(entries) = fs::read_dir(path) {
		let mut entries: Vec<DirEntry> =
			entries.collect::<Result<_, _>>().unwrap_or_else(|_| vec![]);
		sort_entries(&mut entries);
		let total_entries = entries.len();
		for (index, entry) in entries.iter().enumerate() {
			if let Some(entry_name) = entry.file_name().to_str() {
				if entry_name.starts_with(".") && !settings.show_hidden {
					continue;
				}
			}
			if curr_depth == settings.depth {
				return dir_stats;
			}
			let is_last = index == total_entries - 1;
			process_entry(prefix, entry, is_last, curr_depth, settings, &mut dir_stats)
		}
	}

	dir_stats
}

fn main() {
	let settings = Settings::parse();
	let root_path = settings.root_path.clone();

	println!("{}", settings.root_path.color(Color::Blue));
	let stats = print_dir_content(root_path, "", 0, &settings);

	// `stats.dirs + 1` for current directory
	println!("\n{} directories, {} files", stats.dirs + 1, stats.files);
}
