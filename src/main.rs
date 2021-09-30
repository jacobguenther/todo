// File: src/main.rs
// License: GPLv3

use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();

	if args.len() > 1 {
		let command = &args[1];
		match &command[..] {
			"list" => todo::list(),
			"add" => todo::add(&args[2..]),
			"rm" => todo::remove(&args[2..]),
			"done" => todo::done(&args[2..]),
			// "raw" => todo::raw(&args[2..]),
			"sort" => todo::sort(),
			_ => todo::help(),
		}
	} else {
		todo::list();
	}
}
