use crate::state::Result;
use std::process::{
	Command,
	Output,
};

/// Executes git-cliff and returns the output.
pub fn run_git_cliff(args: &[String]) -> Result<String> {
	let output: Output = Command::new("git-cliff").args(args).output()?;
	if output.status.success() {
		let result = String::from_utf8_lossy(&output.stdout).to_string();
		Ok(result)
	} else {
		let e = String::from_utf8_lossy(&output.stderr).to_string();
		Err(e.into())
	}
}
