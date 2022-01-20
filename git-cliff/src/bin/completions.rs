use clap::IntoApp;
use clap_complete::Shell;
use git_cliff::args::Opt;
use std::env;

/// Shell completions can be created with:
/// `cargo run --bin git-cliff-completions`
/// in a directory specified by the environment variable OUT_DIR.
/// See <https://doc.rust-lang.org/cargo/reference/environment-variables.html>
fn main() {
	let out_dir = env::var("OUT_DIR").expect("OUT_DIR is not set");
	let mut app = Opt::into_app();
	for shell in [
		Shell::Bash,
		Shell::Zsh,
		Shell::PowerShell,
		Shell::Fish,
		Shell::Elvish,
	] {
		clap_complete::generate_to(
			shell,
			&mut app,
			env!("CARGO_PKG_NAME"),
			&out_dir,
		)
		.unwrap();
	}
	println!("Completion scripts are generated in {:?}", out_dir);
}
