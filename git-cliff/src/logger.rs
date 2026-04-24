use std::fmt;
use std::sync::atomic::{AtomicUsize, Ordering};

use git_cliff_core::error::{Error, Result};
use indicatif::{ProgressState, ProgressStyle};
use owo_colors::{OwoColorize, Style, Styled};
use tracing::{Event, Level, Span, Subscriber};
use tracing_indicatif::IndicatifLayer;
use tracing_indicatif::span_ext::IndicatifSpanExt;
use tracing_subscriber::fmt::FmtContext;
use tracing_subscriber::fmt::format::{self, FormatEvent, FormatFields};
use tracing_subscriber::layer::{Context, Layer};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Registry};

/// Global variable for storing the maximum width of the modules.
static MAX_MODULE_WIDTH: AtomicUsize = AtomicUsize::new(0);

/// Classic single-cell spinner frames used by indicatif.
const ROOT_SPINNER_TICKS: &[&str] = &["◐", "◓", "◑", "◒"];
/// The previous quarter-circle spinner for nested spans.
const CHILD_SPINNER_TICKS: &[&str] = &["◴", "◷", "◶", "◵"];

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
fn style_level(level: Level) -> Styled<&'static str> {
    match level {
        Level::ERROR => Style::new().red().bold().style("ERROR"),
        Level::WARN => Style::new().yellow().bold().style("WARN"),
        Level::INFO => Style::new().green().bold().style("INFO"),
        Level::DEBUG => Style::new().blue().bold().style("DEBUG"),
        Level::TRACE => Style::new().magenta().bold().style("TRACE"),
    }
}

/// Computes the spinner/elapsed color based on elapsed time.
///
/// The color gradually transitions:
/// - green  -> yellow (0–16s)
/// - yellow -> red    (16–32s)
fn progress_color(state: &ProgressState) -> (u8, u8, u8) {
    let elapsed = state.elapsed().as_secs_f32();
    let t = (elapsed / 32.0).min(1.0);
    if t < 0.5 {
        let nt = t * 2.0;
        (lerp(140, 230, nt), lerp(200, 210, nt), lerp(160, 150, nt))
    } else {
        let nt = (t - 0.5) * 2.0;
        (lerp(230, 230, nt), lerp(210, 140, nt), lerp(150, 140, nt))
    }
}

/// Performs linear interpolation between two color components.
#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
fn lerp(a: u8, b: u8, t: f32) -> u8 {
    ((f32::from(a) + (f32::from(b) - f32::from(a)) * t).clamp(0.0, 255.0)) as u8
}

/// Formats the elapsed time as `X.Ys` (sub-second precision).
fn elapsed_subsec_key(state: &ProgressState, writer: &mut dyn fmt::Write) {
    let seconds = state.elapsed().as_secs();
    let sub_seconds = (state.elapsed().as_millis() % 1000) / 100;
    let (r, g, b) = progress_color(state);
    let _ = write!(
        writer,
        "{}",
        Style::new()
            .truecolor(r, g, b)
            .style(format!("{seconds}.{sub_seconds}s"))
    );
}

/// Formats the current spinner tick and colors it based on elapsed time.
fn spinner_key(state: &ProgressState, writer: &mut dyn fmt::Write, ticks: &'static [&'static str]) {
    let index = ((state.elapsed().as_millis() / 100) as usize) % ticks.len();
    let (r, g, b) = progress_color(state);
    let _ = write!(
        writer,
        "{}",
        Style::new().truecolor(r, g, b).style(ticks[index])
    );
}

fn root_spinner_key(state: &ProgressState, writer: &mut dyn fmt::Write) {
    spinner_key(state, writer, ROOT_SPINNER_TICKS);
}

fn child_spinner_key(state: &ProgressState, writer: &mut dyn fmt::Write) {
    spinner_key(state, writer, CHILD_SPINNER_TICKS);
}

/// Builds the `indicatif::ProgressStyle` used for tracing spans.
///
/// This style:
/// - renders a Unicode spinner for active spans
/// - colorizes the spinner based on elapsed time
/// - shows span name, fields, and wide messages
/// - appends a sub-second elapsed timer
fn indicatif_progress_style(spinner_key: fn(&ProgressState, &mut dyn fmt::Write)) -> ProgressStyle {
    ProgressStyle::with_template(
        "{span_child_prefix}{spinner} {wide_msg} {span_name} {span_fields} [{elapsed_subsec}]",
    )
    .unwrap()
    .with_key("elapsed_subsec", elapsed_subsec_key)
    .with_key("spinner", spinner_key)
}

/// Applies different progress spinner styles to root and nested tracing spans.
struct SpinnerStyleLayer;

impl<S> Layer<S> for SpinnerStyleLayer
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    fn on_enter(&self, id: &tracing::span::Id, ctx: Context<'_, S>) {
        let ticks = if ctx.span(id).is_some_and(|span| span.parent().is_some()) {
            child_spinner_key
        } else {
            root_spinner_key
        };
        Span::current().pb_set_style(&indicatif_progress_style(ticks));
    }
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
        let level = style_level(*metadata.level());
        let target = metadata.target();
        let max_width = max_target_width(target);
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
        .with_progress_style(indicatif_progress_style(root_spinner_key))
        .with_span_child_prefix_symbol("↳ ")
        .with_span_child_prefix_indent(" ");
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_writer(indicatif_layer.get_stderr_writer())
        .with_ansi(true)
        .event_format(GitCliffFormatter);
    let subscriber = Registry::default()
        .with(env_filter)
        .with(indicatif_layer)
        .with(SpinnerStyleLayer)
        .with(fmt_layer);
    subscriber
        .try_init()
        .map_err(|e| Error::LoggerError(e.to_string()))
}
