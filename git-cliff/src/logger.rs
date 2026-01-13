use std::fmt;
use std::sync::atomic::{AtomicUsize, Ordering};

use git_cliff_core::error::{Error, Result};
use indicatif::{ProgressState, ProgressStyle};
use owo_colors::{OwoColorize, Style, Styled};
use tracing::{Event, Level, Subscriber};
use tracing_indicatif::IndicatifLayer;
use tracing_subscriber::fmt::format::{self, FormatEvent, FormatFields};
use tracing_subscriber::fmt::{FmtContext, FormattedFields};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Registry};

/// Global variable for storing the maximum width of the modules.
static MAX_MODULE_WIDTH: AtomicUsize = AtomicUsize::new(0);

/// Unicode braille spinner frames used by indicatif.
const SPINNER_TICKS: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

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

/// Adds styles/colors to the given level and returns it.
fn style_level(level: &Level) -> Styled<&'static str> {
    match *level {
        Level::ERROR => Style::new().red().bold().style("ERROR"),
        Level::WARN => Style::new().yellow().bold().style("WARN"),
        Level::INFO => Style::new().green().bold().style("INFO"),
        Level::DEBUG => Style::new().blue().bold().style("DEBUG"),
        Level::TRACE => Style::new().magenta().bold().style("TRACE"),
    }
}

/// Shortens the target string to fit within the specified width.
/// TODO: This function is currently unused but kept for future.
#[allow(dead_code)]
fn shorten_target(target: &str, width: usize) -> String {
    if target.len() <= width {
        return target.to_string();
    }
    let parts: Vec<&str> = target.split("::").collect();
    if parts.len() >= 2 {
        format!("{}...{}", parts[0], parts[parts.len() - 1])
    } else {
        target.to_string()
    }
}

/// Formats the elapsed time as `X.Ys` (sub-second precision).
fn elapsed_subsec_key(state: &ProgressState, writer: &mut dyn fmt::Write) {
    let seconds = state.elapsed().as_secs();
    let sub_seconds = (state.elapsed().as_millis() % 1000) / 100;
    let _ = write!(writer, "{}.{}s", seconds, sub_seconds);
}

/// Emits an ANSI color escape sequence for the spinner, based on elapsed time.
///
/// The color gradually transitions:
/// - green  -> yellow (0–4s)
/// - yellow -> red    (4–8s)
fn color_start_key(state: &ProgressState, writer: &mut dyn fmt::Write) {
    let elapsed = state.elapsed().as_secs_f32();
    let t = (elapsed / 8.0).min(1.0); // 8秒で変化
    let (r, g, b) = if t < 0.5 {
        let nt = t * 2.0;
        (lerp(140, 230, nt), lerp(200, 210, nt), lerp(160, 150, nt))
    } else {
        let nt = (t - 0.5) * 2.0;
        (lerp(230, 230, nt), lerp(210, 140, nt), lerp(150, 140, nt))
    };
    let _ = write!(writer, "\x1b[38;2;{};{};{}m", r, g, b);
}

/// Performs linear interpolation between two color components.
fn lerp(a: u8, b: u8, t: f32) -> u8 {
    (a as f32 + (b as f32 - a as f32) * t) as u8
}

/// Resets ANSI styling to the terminal default.
///
/// This must be paired with `color_start_key` to avoid leaking color state into subsequent log
/// output.
fn color_end_key(_: &ProgressState, writer: &mut dyn fmt::Write) {
    let _ = writer.write_str("\x1b[0m");
}

/// Emits an ANSI escape sequence for dim (muted) foreground text.
fn dim_start_key(_: &ProgressState, writer: &mut dyn fmt::Write) {
    let _ = writer.write_str("\x1b[90m");
}

/// Resets ANSI styling to the terminal default.
///
/// This must be paired with `dim_start_key` to avoid leaking color state into subsequent log
/// output.
fn dim_end_key(_: &ProgressState, writer: &mut dyn fmt::Write) {
    let _ = writer.write_str("\x1b[0m");
}

/// Builds the `indicatif::ProgressStyle` used for tracing spans.
///
/// This style:
/// - renders a Unicode spinner for active spans
/// - colorizes the spinner based on elapsed time
/// - shows span name, fields, and wide messages
/// - appends a sub-second elapsed timer
fn indicatif_progress_style() -> ProgressStyle {
    ProgressStyle::with_template(
        "{span_child_prefix}{color_start}{spinner}{color_end} {dim_start}{span_name} \
         {span_fields} {wide_msg}{dim_end} [{color_start}{elapsed_subsec}{color_end}]",
    )
    .unwrap()
    .with_key("elapsed_subsec", elapsed_subsec_key)
    .with_key("color_start", color_start_key)
    .with_key("color_end", color_end_key)
    .with_key("dim_start", dim_start_key)
    .with_key("eim_end", dim_end_key)
    .tick_strings(SPINNER_TICKS)
}

/// Simple formatter. We format: "LEVEL TARGET > MESSAGE", with a basic padding for target.
struct GitCliffFormatter;

impl<S, N> FormatEvent<S, N> for GitCliffFormatter
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        mut writer: format::Writer<'_>,
        event: &Event<'_>,
    ) -> fmt::Result {
        let metadata = event.metadata();
        let level = style_level(metadata.level());
        let target = metadata.target();
        let max_width = max_target_width(&target);
        write!(
            &mut writer,
            "{} {} > ",
            Padded {
                value: level,
                width: 5,
            },
            Padded {
                value: target.bright_black().bold(),
                width: max_width,
            },
        )?;
        if let Some(scope) = ctx.event_scope() {
            for span in scope.from_root() {
                write!(writer, "{}", span.name().bright_black().bold())?;
                let ext = span.extensions();
                let fields = &ext
                    .get::<FormattedFields<N>>()
                    .expect("will never be `None`");
                if !fields.is_empty() {
                    write!(writer, "{{{}}}", fields)?;
                }
                write!(writer, "{}", ": ".bright_black().bold())?;
            }
        }
        ctx.field_format().format_fields(writer.by_ref(), event)?;
        writeln!(writer)
    }
}

/// Initializes the global tracing subscriber.
pub fn init() -> Result<()> {
    // Build EnvFilter from `RUST_LOG` or fallback to "info"
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into());
    let indicatif_layer = IndicatifLayer::new()
        .with_progress_style(indicatif_progress_style())
        .with_span_child_prefix_symbol("↳ ")
        .with_span_child_prefix_indent(" ");
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_writer(indicatif_layer.get_stderr_writer())
        .with_ansi(true)
        .event_format(GitCliffFormatter);
    let subscriber = Registry::default()
        .with(env_filter)
        .with(indicatif_layer)
        .with(fmt_layer);
    subscriber
        .try_init()
        .map_err(|e| Error::LoggerError(e.to_string()))
}
