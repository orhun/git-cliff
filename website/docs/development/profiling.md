---
sidebar_position: 1
---
# Performance Profiling

**git-cliff** can be built with performance profiling instrumentation, which helps with finding bottlenecks.

The profiler can be enabled via the `profiler` feature and the `bench` build profile.

```bash
cargo build --profile=bench --features=profiler
```

To create a flame graph SVG:

```bash
cargo run --profile=bench --features=profiler
```

e.g.

![flamegraph example](https://github.com/user-attachments/assets/d29339a5-1c82-4630-bcb8-0b3466d8710a)
