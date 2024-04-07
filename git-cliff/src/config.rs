use crate::args::Opt;
#[allow(deprecated)]
use git_cliff_core::config::models_v1::Config as Config_v1;
use git_cliff_core::config::{
	embed::BuiltinConfig,
	embed::EmbeddedConfig,
	models_v2::Config as Config_v2,
	DEFAULT_CONFIG_FILENAME,
	DEFAULT_CONFIG_VERSION,
};
use git_cliff_core::error::{
	Error,
	Result,
};
use std::fs;
use std::path::PathBuf;

/// Gets the effective config argument.
fn get_effective_config_arg(path: PathBuf) -> PathBuf {
	if !path.exists() {
		if let Some(config_path) = dirs::config_dir().map(|dir| {
			dir.join(env!("CARGO_PKG_NAME"))
				.join(DEFAULT_CONFIG_FILENAME)
		}) {
			return config_path;
		}
	}
	path
}

/// Determines the version of the config schema.
fn determine_config_version(args: &Opt, raw_config: toml::Table) -> i64 {
	// determine if `meta.version` is set in the config file.
	let meta_version_option = raw_config
		.get("meta")
		.and_then(|val| val.as_table())
		.and_then(|table| table.get("version"))
		.and_then(|version| version.as_integer());

	// `--config-version` takes precedence over `meta.version`.
	if let Some(args_version) = args.config_version {
		if let Some(meta_version) = meta_version_option {
			warn!(
				"Argument `--config-version = {args_version}` takes precedence \
				 over option `meta.version = {meta_version}`."
			);
		}
		return args_version;
	} else {
		return meta_version_option.unwrap_or(DEFAULT_CONFIG_VERSION);
	};
}

/// Loads the configuration based on the given command line arguments.
pub fn load_config(args: &Opt) -> Result<Config_v2> {
	let config_arg = get_effective_config_arg(args.config.clone());

	// If `--config` matches the name of a built-in config, load it.
	let config_str = if let Ok((builtin_config, builtin_config_name)) =
		BuiltinConfig::get_config_str(config_arg.to_string_lossy().to_string())
	{
		info!("Using built-in configuration {builtin_config_name}.");
		builtin_config
	}
	// If `--config` denotes an existing file, load it from there.
	else if config_arg.is_file() {
		info!(
			"Loading configuration from {}.",
			config_arg.to_string_lossy()
		);
		fs::read_to_string(config_arg)?
	}
	// If the manifest contains a config, load it.
	else if let Some(contents) =
		git_cliff_core::config::embed::read_from_manifest()?
	{
		info!("Loading configuration from manifest.");
		contents
	}
	// Otherwise fall back to using the embedded configuration from
	// ./config/cliff.toml.
	else {
		warn!(
			"{:?} could not be found. Using the default configuration.",
			args.config
		);
		EmbeddedConfig::get_config_str()?
	};

	// Determine the version of the config based on the cli argument
	// `--config-version` and the option `meta.version`.
	let raw_config = config_str.parse::<toml::Table>()?;
	let config_version = determine_config_version(args, raw_config);
	info!("Loading configuration version {config_version}.");

	// load the file using https://docs.rs/config
	let raw_config = config::Config::builder()
		.add_source(config::File::from_str(
			&config_str,
			config::FileFormat::Toml,
		))
		.add_source(config::Environment::with_prefix("GIT_CLIFF").separator("__"))
		.build()?;

	// Turn the toml::Table into the proper config struct.
	if config_version == 1 {
		let config_v1 = raw_config.try_deserialize::<Config_v1>()?;
		return Ok(Config_v2::from(config_v1));
	} else if config_version == 2 {
		return Ok(raw_config.try_deserialize::<Config_v2>()?);
	} else {
		return Err(Error::ArgumentError(format!(
			"Configuration version {} is not supported.",
			config_version
		)));
	}
}
