use git_cliff_core::error::Result;

/// Creates a profiler guard and returns it.
pub(crate) fn start_profiling() -> Option<pprof::ProfilerGuard<'static>> {
	match pprof::ProfilerGuardBuilder::default()
		.frequency(1000)
		.blocklist(&["libc", "libgcc", "pthread", "vdso"])
		.build()
	{
		Ok(guard) => Some(guard),
		Err(e) => {
			log::error!("failed to build profiler guard: {e}");
			None
		}
	}
}

/// Reports the profiling results.
pub(crate) fn finish_profiling(
	profiler_guard: Option<pprof::ProfilerGuard>,
) -> Result<()> {
	match profiler_guard
		.expect("failed to retrieve profiler guard")
		.report()
		.build()
	{
		Ok(report) => {
			#[cfg(feature = "profiler-flamegraph")]
			{
				use std::fs::File;
				let random = rand::random::<u64>();
				let file = File::create(format!(
					"{}.{random}.flamegraph.svg",
					env!("CARGO_PKG_NAME"),
				))?;
				if let Err(e) = report.flamegraph(file) {
					log::error!("failed to create flamegraph file: {e}");
				}
			}

			#[cfg(not(feature = "profiler-flamegraph"))]
			{
				log::info!("profiling report: {:?}", &report);
			}
		}
		Err(e) => {
			log::error!("failed to build profiler report: {e}");
		}
	}

	Ok(())
}
