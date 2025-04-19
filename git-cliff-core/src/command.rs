use crate::error::Result;
use std::io::{
	Error as IoError,
	Write,
};
use std::process::{
	Command,
	Stdio,
};
use std::thread;
use std::{
	env,
	str,
};

/// Runs the given OS command and returns the output as string.
///
/// Use `input` parameter to specify a text to write to stdin.
/// Environment variables are set accordingly to `envs`.
pub fn run(
	command: &str,
	input: Option<String>,
	envs: Vec<(&str, &str)>,
) -> Result<String> {
	log::trace!("Running command: {:?}", command);
	let mut child = if cfg!(target_os = "windows") {
		Command::new("cmd")
			.envs(envs)
			.args(["/C", command])
			.stdin(Stdio::piped())
			.stdout(Stdio::piped())
			.current_dir(env::current_dir()?)
			.spawn()
	} else {
		Command::new("sh")
			.envs(envs)
			.args(["-c", command])
			.stdin(Stdio::piped())
			.stdout(Stdio::piped())
			.current_dir(env::current_dir()?)
			.spawn()
	}?;
	if let Some(input) = input {
		let mut stdin = child
			.stdin
			.take()
			.ok_or_else(|| IoError::other("stdin is not captured"))?;
		thread::spawn(move || {
			stdin
				.write_all(input.as_bytes())
				.expect("Failed to write to stdin");
		});
	}
	let output = child.wait_with_output()?;
	if output.status.success() {
		Ok(str::from_utf8(&output.stdout)?.to_string())
	} else {
		for output in [output.stdout, output.stderr] {
			let output = str::from_utf8(&output)?.to_string();
			if !output.is_empty() {
				log::error!("{}", output);
			}
		}
		Err(
			IoError::other(format!("command exited with {:?}", output.status))
				.into(),
		)
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	#[cfg(target_family = "unix")]
	fn run_os_command() -> Result<()> {
		assert_eq!(
			"eroc-ffilc-tig",
			run("echo $APP_NAME | rev", None, vec![(
				"APP_NAME",
				env!("CARGO_PKG_NAME")
			)])?
			.trim()
		);
		assert_eq!(
			"eroc-ffilc-tig",
			run("rev", Some(env!("CARGO_PKG_NAME").to_string()), vec![])?.trim()
		);
		assert_eq!("testing", run("echo 'testing'", None, vec![])?.trim());
		assert!(run("some_command", None, vec![]).is_err());
		Ok(())
	}
}
