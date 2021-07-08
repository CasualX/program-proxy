use std::{env, fs, io, path::Path, process};

fn main() {
	let mut base_path = env::current_exe()
		.expect("Error getting the current executable's path");

	// Find the configuration file as the current executable's with its extension replaced by cfg
	let mut cfg_path = base_path.clone();
	cfg_path.set_extension("cfg");

	// The base path is the directory of our current executable
	base_path.pop();

	// Read the configuration file
	let lines = match fs::read_to_string(&cfg_path) {
		Ok(cfg) => cfg,
		Err(err) => {
			let file_name = cfg_path.file_name();
			if err.kind() == io::ErrorKind::NotFound {
				panic!("The configuration file is missing: {:?}", file_name);
			}
			else {
				panic!("Error reading the configuration file {:?}: {:?}", file_name, err);
			}
		},
	};
	// Into lines
	let mut lines = lines.lines();

	// Read the first line, it must be the program line
	let program = lines.next().expect("The configuration file looks empty!");
	if !program.starts_with("program=") {
		panic!("The configuration file must start by declaring a path to the program.");
	}

	// Make the program path absolute if it is relative
	let program_buf;
	let mut program = Path::new(&program[8..]);
	if program.is_relative() {
		program_buf = base_path.join(program);
		program = &program_buf;
	}

	// Build the process command
	let mut command = process::Command::new(program);

	// Enforce the current dir can only be set once
	let mut set_current_dir = false;

	for line in lines {
		// Comments and empty lines
		if line.trim().is_empty() || line.starts_with("#") {
			continue;
		}
		// Handle current directory
		else if line.starts_with("current_dir=") {
			if set_current_dir {
				panic!("The current directory is set more than once!");
			}

			// Make the current_dir path absolute if it is relative
			let current_dir_buf;
			let mut current_dir = Path::new(&line[12..]);
			if current_dir.is_relative() {
				current_dir_buf = base_path.join(current_dir);
				current_dir = &current_dir_buf;
			}

			command.current_dir(current_dir);
			set_current_dir = true;
		}
		// Insert a custom argument
		else if line.starts_with("arg=") {
			let arg = &line[4..];
			command.arg(arg);
		}
		// Add an environment variable
		else if line.starts_with("env=") {
			let env = &line[4..];
			let split_at = env.find("=")
				.expect("Environment argument syntax error, expecting KEY=VALUE");
			let key = &env[..split_at];
			let value = &env[split_at + 1..];
			command.env(key, value);
		}
		// Remove an environment variable
		else if line.starts_with("env_remove=") {
			let env_remove = &line[11..];
			command.env_remove(env_remove);
		}
		// Clear all environment variables
		else if line == "env_clear" {
			command.env_clear();
		}
		else {
			panic!("Configuration syntax error: {}", line);
		}
	}

	// Append the command-line arguments
	// Skip the first argument which is typically the path to the invoked executable
	// TODO! Make this an option?
	let mut args = env::args_os();
	args.next();
	command.args(args);

	// Spawn the child
	let mut child = command.spawn()
		.expect("There was an error spawning the child process");

	// Wait for the child to exit
	let exit_status = child.wait()
		.expect("Failed to wait for the child process");

	// Exit with the child's exit status code
	let code = exit_status.code().unwrap_or(255);
	process::exit(code);
}
