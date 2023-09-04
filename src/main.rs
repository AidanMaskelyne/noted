use ansi_term::{Colour, Style};
use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
	#[command(subcommand)]
	action: Option<ActionOptions>,
}

#[derive(Subcommand)]
enum ActionOptions {
	/// Adds a new note
	Add {
		/// Name of new note; will default to current system time
		name: Option<String>,
	},
}

struct Todo {
	index: i32,
	title: String,
	completed: bool,
}

fn main() -> Result<()> {
	// In the case that no args are passed, just print the home page and exit
	if std::env::args().len() == 1 {
		print_home_page();
		return Ok(());
	}

	let args = Args::parse();

	match &args.action {
		Some(ActionOptions::Add { name }) => {}
		None => {}
	}

	return Ok(());
}

fn print_home_page() {
	let style = Style::new().bold();
	let todos = get_all_todos();

	println!(
		"{}	{}	{}",
		Style::new().italic().underline().paint("Index"),
		Style::new().italic().underline().paint("Completed"),
		Style::new().italic().underline().paint("Todo")
	);
	for todo in todos.iter() {
		println!(
			"[{}]	{}		{}",
			Colour::Blue.paint(todo.index.to_string()),
			if todo.completed {
				Colour::Green.paint("yes")
			} else {
				Colour::Red.paint("no")
			},
			style.paint(todo.title.to_string())
		);
	}
}

/// Returns all todos, sorted by (ascending) index
fn get_all_todos() -> Vec<Todo> {
	vec![
		Todo {
			index: 1,
			title: String::from("Wash dishes"),
			completed: false,
		},
		Todo {
			index: 2,
			title: String::from("Fetch laundry in"),
			completed: true,
		},
	]
}
