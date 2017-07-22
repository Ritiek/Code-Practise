use std::process::Command;
use std::io::stdout;
use std::io::Write;

/*
 * let _ = blah(); to hide unused variable warnings
 * stdout().flush(); to stop ping output from printing before "Hello World"
 */

fn main() {
	let output = Command::new("ping")
						.arg("-c 1")
						.arg("google.com")
						.output()
						.expect("Nah.");

	let _ = Command::new("echo")
			.arg("Hello World")
			.spawn();

	let _ = stdout().flush();

	println!("{}",
	String::from_utf8_lossy(&output.stdout)
	);

	println!();
}
