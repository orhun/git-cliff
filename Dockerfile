# syntax=docker/dockerfile:1.4.3-labs
FROM lukemathwalker/cargo-chef:0.1.47-rust-1.65.0-buster AS chef
WORKDIR app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
ENV CARGO_NET_GIT_FETCH_WITH_CLI=true
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --locked --no-default-features
RUN rm -f target/release/deps/git_cliff*

FROM debian:buster-slim as runner
COPY --from=builder /app/target/release/git-cliff /usr/local/bin
WORKDIR git-home
RUN cat <<'EOF' > entrypoint.sh
#!/bin/sh
cp -r /app /git-home/app
cd /git-home/app
exec git-cliff "$@"
EOF
ENTRYPOINT ["sh", "entrypoint.sh"]
