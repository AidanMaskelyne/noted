use ansi_term::{Colour, Style};
use anyhow::Result;
use clap::{Parser, Subcommand};
use console::Term;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
	#[command(subcommand)]
	command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
	/// Interact with your todos
	Todos {
		#[command(subcommand)]
		action: TodoActions,
	},

	/// Interact with your notes
	Notes {
		#[command(subcommand)]
		action: NoteActions,
	},
}

#[derive(Debug, Subcommand)]
enum TodoActions {
	/// Add a new todo
	New {
		/// Name of new todo
		name: String,
	},

	/// Marks a todo as completed
	Do {
		/// Index of the todo to be marked as completed
		index: i32,
	},
}

#[derive(Subcommand)]
enum NoteActions {
	/// Creates a new note
	New {
		/// Optional name of new note (defaults to system time)
		name: Option<String>,
	},
}

#[derive(Debug, Clone)]
struct Todo {
	index: i32,
	title: String,
	completed: bool,
}

// Eventually index will not need to be passed to the constructor, it will be calculated from exists todos.
// This is just for testing purposes
impl Todo {
	fn new(index: i32, title: String) -> Todo {
		Todo {
			index: index,
			title: title,
			completed: false,
		}
	}
	fn mark_as_completed(&mut self) {
		self.completed = true;
		write_todo(self.clone());
	}
}

fn main() -> Result<()> {
	// In the case that no args are passed, just print the home page and exit
	if std::env::args().len() == 1 {
		print_home_page();
		return Ok(());
	}

	let args = Args::parse();

	match &args.command {
		Some(Commands::Todos { action }) => match action {
			TodoActions::New { name } => {
				create_new_todo(name.clone());
			}
			TodoActions::Do { index } => {
				let mut completed_todo = get_todo(index);
				completed_todo.mark_as_completed();
			}
		},
		Some(Commands::Notes { action }) => match action {
			NoteActions::New { name } => match name {
				Some(name) => println!("Creating note with title: {name}"),
				None => println!("Creating note with default title"),
			},
		},
		None => {}
	}

	return Ok(());
}

/// Prints the home page when the application is run without any arguments
fn print_home_page() {
	let term = Term::stdout();
	let mut console_divider = String::new();

	if let Some((term_width, term_height)) = term_size::dimensions() {
		term.write_line(
			console::pad_str(
				&Colour::Blue.paint("Home").to_string() as &str,
				term_width,
				console::Alignment::Center,
				None,
			)
			.as_ref(),
		);

		for _ in 0..term_width {
			console_divider.push_str("-");
		}
	}

	println!("{}", Style::new().dimmed().paint(&console_divider));

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
			// Bold does not work on all terminals
			Style::new().bold().paint(todo.title.to_string())
		);
	}

	println!("{}", Style::new().dimmed().paint(&console_divider));
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
		Todo::new(3, String::from("Make dinner")),
	]
}

fn setup_todos_storage(path: Option<PathBuf>) {
	todo!();
}

fn create_new_todo(todo_name: String) {
	println!("Creating new todo \"{todo_name}\"");
}

/// Reads the stored todos and searches for the one that matches the supplied index
fn get_todo(index: &i32) -> Todo {
	Todo::new(4, String::from("wasd"))
}

/// Write the provided Todo object into the storage location
fn write_todo(todo: Todo) {}
