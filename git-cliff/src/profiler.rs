pub(crate) fn start_profiling() -> pprof::ProfilerGuard<'static> {
	return pprof::ProfilerGuardBuilder::default()
		.frequency(1000)
		.blocklist(&["libc", "libgcc", "pthread", "vdso"])
		.build()
		.unwrap();
}

pub(crate) fn finish_profiling(profiler_guard: pprof::ProfilerGuard) {
	match profiler_guard.report().build() {
		Ok(report) => {
			#[cfg(feature = "profiler-flamegraph")]
			{
				use std::fs::File;
				let random = rand::random::<u64>();

				match File::create(format!(
					"{}.{}.flamegraph.svg",
					"git-cliff", random
				)) {
					Ok(file) => {
						report.flamegraph(file).unwrap();
					}
					Err(err) => {
						log::error!("Failed to create flamegraph file {}", err);
					}
				}
			}

			#[cfg(not(feature = "profiler-flamegraph"))]
			{
				log::info!("profiling report: {:?}", &report);
			}
		}
		Err(err) => {
			log::error!("Failed to build profiler report {}", err);
		}
	}
}
