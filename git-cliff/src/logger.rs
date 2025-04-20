use env_logger::{
	Builder,
	fmt::{
		Color,
		Style,
		StyledValue,
	},
};
use git_cliff_core::error::{
	Error,
	Result,
};
#[cfg(feature = "remote")]
use indicatif::{
	ProgressBar,
	ProgressStyle,
};
use log::Level;
use std::io::Write;
use std::sync::atomic::{
	AtomicUsize,
	Ordering,
};
use std::{
	env,
	fmt,
};

/// Environment variable to use for the logger.
const LOGGER_ENV: &str = "RUST_LOG";

/// Global variable for storing the maximum width of the modules.
static MAX_MODULE_WIDTH: AtomicUsize = AtomicUsize::new(0);

/// Wrapper for the padded values.
struct Padded<T> {
	value: T,
	width: usize,
}

impl<T: fmt::Display> fmt::Display for Padded<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{: <width$}", self.value, width = self.width)
	}
}

/// Returns the max width of the target.
fn max_target_width(target: &str) -> usize {
	let max_width = MAX_MODULE_WIDTH.load(Ordering::Relaxed);
	if max_width < target.len() {
		MAX_MODULE_WIDTH.store(target.len(), Ordering::Relaxed);
		target.len()
	} else {
		max_width
	}
}

/// Adds colors to the given level and returns it.
fn colored_level(style: &mut Style, level: Level) -> StyledValue<'_, &'static str> {
	match level {
		Level::Trace => style.set_color(Color::Magenta).value("TRACE"),
		Level::Debug => style.set_color(Color::Blue).value("DEBUG"),
		Level::Info => style.set_color(Color::Green).value("INFO "),
		Level::Warn => style.set_color(Color::Yellow).value("WARN "),
		Level::Error => style.set_color(Color::Red).value("ERROR"),
	}
}

#[cfg(feature = "remote")]
lazy_static::lazy_static! {
	/// Lazily initialized progress bar.
	pub static ref PROGRESS_BAR: ProgressBar = {
		let progress_bar = ProgressBar::new_spinner();
		progress_bar.set_style(
			ProgressStyle::with_template("{spinner:.green} {msg}")
				.unwrap()
				.tick_strings(&[
					"▹▹▹▹▹",
					"▸▹▹▹▹",
					"▹▸▹▹▹",
					"▹▹▸▹▹",
					"▹▹▹▸▹",
					"▹▹▹▹▸",
					"▪▪▪▪▪",
				]),
		);
		progress_bar
	};
}

/// Initializes the global logger.
///
/// This method also creates a progress bar which is triggered
/// by the network operations that are related to GitHub.
#[allow(unreachable_code, clippy::needless_return)]
pub fn init() -> Result<()> {
	let mut builder = Builder::new();
	builder.format(move |f, record| {
		let target = record.target();
		let max_width = max_target_width(target);

		let mut style = f.style();
		let level = colored_level(&mut style, record.level());

		let mut style = f.style();
		let target = style.set_bold(true).value(Padded {
			value: target,
			width: max_width,
		});

		#[cfg(feature = "github")]
		{
			let message = record.args().to_string();
			if message
				.starts_with(git_cliff_core::remote::github::START_FETCHING_MSG)
			{
				PROGRESS_BAR
					.enable_steady_tick(std::time::Duration::from_millis(80));
				PROGRESS_BAR.set_message(message);
				return Ok(());
			} else if message
				.starts_with(git_cliff_core::remote::github::FINISHED_FETCHING_MSG)
			{
				PROGRESS_BAR.finish_and_clear();
				return Ok(());
			}
		}

		#[cfg(feature = "gitlab")]
		{
			let message = record.args().to_string();
			if message
				.starts_with(git_cliff_core::remote::gitlab::START_FETCHING_MSG)
			{
				PROGRESS_BAR
					.enable_steady_tick(std::time::Duration::from_millis(80));
				PROGRESS_BAR.set_message(message);
				return Ok(());
			} else if message
				.starts_with(git_cliff_core::remote::gitlab::FINISHED_FETCHING_MSG)
			{
				PROGRESS_BAR.finish_and_clear();
				return Ok(());
			}
		}

		#[cfg(feature = "gitea")]
		{
			let message = record.args().to_string();
			if message.starts_with(git_cliff_core::remote::gitea::START_FETCHING_MSG)
			{
				PROGRESS_BAR
					.enable_steady_tick(std::time::Duration::from_millis(80));
				PROGRESS_BAR.set_message(message);
				return Ok(());
			} else if message
				.starts_with(git_cliff_core::remote::gitea::FINISHED_FETCHING_MSG)
			{
				PROGRESS_BAR.finish_and_clear();
				return Ok(());
			}
		}

		#[cfg(feature = "bitbucket")]
		{
			let message = record.args().to_string();
			if message
				.starts_with(git_cliff_core::remote::bitbucket::START_FETCHING_MSG)
			{
				PROGRESS_BAR
					.enable_steady_tick(std::time::Duration::from_millis(80));
				PROGRESS_BAR.set_message(message);
				return Ok(());
			} else if message.starts_with(
				git_cliff_core::remote::bitbucket::FINISHED_FETCHING_MSG,
			) {
				PROGRESS_BAR.finish_and_clear();
				return Ok(());
			}
		}

		writeln!(f, " {} {} > {}", level, target, record.args())
	});

	if let Ok(var) = env::var(LOGGER_ENV) {
		builder.parse_filters(&var);
	}

	builder
		.try_init()
		.map_err(|e| Error::LoggerError(e.to_string()))
}
