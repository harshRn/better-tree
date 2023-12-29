use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "Better tree")]
#[command(author = "Artem S. <https://github.com/tesla33io>")]
#[command(version = "0.1.0")]
#[command(about = "Better `tree` command")]
pub struct Settings {
	/// The path to the scan directory
	#[arg(default_value = ".")]
	pub root_path:		String,

	/// Directory scanning depth
	#[arg(short, long, default_value = "3")]
	pub depth:			i32,

	/// Show hidden entries
	#[arg(short, long, default_value = "false")]
	pub show_hidden:	bool,
}
