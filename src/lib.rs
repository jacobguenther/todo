use colored::*;
use std::fs::OpenOptions;
use std::io::prelude::Read;
use std::io::{
	BufReader,
	BufWriter,
	Write,
};
use std::{
	env,
	path,
	process,
};

const DONE_SYMBOL: &str = "[*] ";
const NOT_DONE_SYMBOL: &str = "[ ] ";
fn symbol_and_task(line: &str) -> (&str, &str) {
	(&line[..4], &line[4..])
}

fn get_todofile(
	read: bool,
	write: bool,
	append: bool,
	truncate: bool,
	create: bool,
) -> std::fs::File {
	let mut home = env::var_os("XDG_DATA_HOME")
		.unwrap_or_else(|| env::var_os("HOME").unwrap());

	if cfg!(windows) {
		home = env::var_os("USERPROFILE").unwrap();
	}

	let todo = path::Path::new("TODO");
	let home_path = home.to_str().unwrap();

	let path = path::Path::new(&home_path).join(todo);

	OpenOptions::new()
		.read(read)
		.write(write)
		.append(append)
		.truncate(truncate)
		.create(create)
		.open(path)
		.expect("Couldn't open the todofile")
}
fn read_contents() -> String {
	let read = true;
	let write = true;
	let create = true;
	let todofile = get_todofile(read, write, false, false, create);
	let mut contents = String::new();
	let _ = BufReader::new(&todofile)
		.read_to_string(&mut contents)
		.expect("Couldn't read todofile");
	contents
}
fn read_todos() -> Vec<String> {
	read_contents().lines().map(|s| s.to_owned()).collect()
}

pub fn list() {
	let mut print_buffer = String::new();

	for (number, line) in read_contents().lines().enumerate() {
		// Converts number into BOLD string
		let number = (number + 1).to_string().bold();

		// Saves the symbol of current task
		// Saves a task without a symbol
		let (symbol, task) = symbol_and_task(line);

		print_buffer.push_str(&number);
		print_buffer.push(' ');
		// Checks if the current task is completed or not...
		if symbol == DONE_SYMBOL {
			// DONE
			// If the task is completed, then it prints it with a strikethrough
			let s = format!("{}", task.strikethrough());
			print_buffer.push_str(&s);
		} else if symbol == NOT_DONE_SYMBOL {
			// NOT DONE
			// If the task is not completed yet, then it will print it as it is
			print_buffer.push_str(task);
		}
		print_buffer.push('\n');
	}
	println!("{}", print_buffer)
}

pub fn add(args: &[String]) {
	if args.is_empty() {
		eprintln!("todo add takes at least 1 argument");
		process::exit(1);
	} else {
		let read = true;
		let write = true;
		let append = true;
		let create = true;
		let todofile = get_todofile(read, write, append, false, create);
		let mut buffer = BufWriter::new(todofile);

		for arg in args {
			if arg.trim().is_empty() {
				continue;
			}

			let mut line = String::from(NOT_DONE_SYMBOL);
			line.push_str(arg);
			line.push('\n');

			buffer
				.write_all(line.as_bytes())
				.expect("unable to write data");
		}

		// Appends a new task/s to the file
	}
}

// Removes a task
pub fn remove(args: &[String]) {
	if args.is_empty() {
		eprintln!("todo rm takes at least 1 argument");
		process::exit(1);
	} else {
		let todos: Vec<_> = read_todos();
		let write = true;
		let truncate = true;
		let todofile = get_todofile(false, write, false, truncate, false);
		let mut buffer = BufWriter::new(todofile);

		for (pos, line) in todos.iter().enumerate() {
			let index_str = (pos + 1).to_string();
			if args.contains(&index_str) {
				continue;
			}

			let mut line = line.to_owned();
			line.push('\n');

			buffer
				.write_all(line.as_bytes())
				.expect("unable to write data");
		}
	}
}

// Sorts done tasks
pub fn sort() {
	let mut todo = String::new();
	let mut done = String::new();

	let todos: Vec<_> = read_todos();

	for line in todos.iter() {
		if line.len() >= 5 {
			let (symbol, _) = symbol_and_task(line);
			if symbol == NOT_DONE_SYMBOL {
				todo.push_str(line);
				todo.push('\n');
			} else if symbol == DONE_SYMBOL {
				done.push_str(line);
				done.push('\n');
			}
		}
	}

	// Opens the TODO file with a permission to:
	let write = true;
	let truncate = true;
	let mut todofile = get_todofile(false, write, false, truncate, false);

	todo.push_str(&done);

	// Writes contents of a todo variable into the TODO file
	todofile
		.write_all(todo.as_bytes())
		.expect("Error while trying to save the todofile");
}

pub fn done(args: &[String]) {
	if args.is_empty() {
		eprintln!("todo done takes at least 1 argument");
		process::exit(1);
	} else {
		let todos = read_todos();
		// Opens the TODO file with a permission to overwrite it
		let write = true;
		let todofile = get_todofile(false, write, false, false, false);
		let mut buffer = BufWriter::new(todofile);

		let mut line_buffer = String::new();
		for (pos, line) in todos.iter().enumerate() {
			if line.len() >= 5 {
				let (symbol, task) = symbol_and_task(line);

				let index_str = (pos + 1).to_string();
				if args.contains(&index_str) {
					if symbol == NOT_DONE_SYMBOL {
						line_buffer.push_str(DONE_SYMBOL);
						line_buffer.push_str(&task);
						line_buffer.push('\n');
						buffer
							.write_all(line_buffer.as_bytes())
							.expect("unable to write data");
						line_buffer.clear();
					} else if symbol == DONE_SYMBOL {
						line_buffer.push_str(NOT_DONE_SYMBOL);
						line_buffer.push_str(&task);
						line_buffer.push('\n');
						buffer
							.write_all(line_buffer.as_bytes())
							.expect("unable to write data");
						line_buffer.clear();
					}
				} else if symbol == NOT_DONE_SYMBOL || symbol == DONE_SYMBOL {
					line_buffer.push_str(&line);
					line_buffer.push('\n');
					buffer
						.write_all(line_buffer.as_bytes())
						.expect("unable to write data");
					line_buffer.clear();
				}
			}
		}
	}
}

const TODO_HELP: &str = "Usage: todo [COMMAND] [ARGUMENTS]
Todo is a super fast and simple tasks organizer written in rust
Example: todo list
Available commands:
    - add [TASK/s] 
        adds new task/s
        Example: todo add \"buy carrots\"
    - list
        lists all tasks
        Example: todo list
    - done [INDEX]
        marks task as done
        Example: todo done 2 3 (marks second and third tasks as completed)
    - rm [INDEX] 
        removes a task
        Example: todo rm 4 
    - sort
        sorts completed and uncompleted tasks
        Example: todo sort 
    - raw [todo/done]
        prints nothing but done/incompleted tasks in plain text, useful for scripting
        Example: todo raw done
";

pub fn help() {
	// For readability
	println!("{}", TODO_HELP);
}
