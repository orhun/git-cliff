FROM lukemathwalker/cargo-chef:0.1.39-rust-1.63.0-slim-buster as planner
WORKDIR app
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM lukemathwalker/cargo-chef:0.1.39-rust-1.63.0-slim-buster as cacher
WORKDIR app
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    --allow-unauthenticated git
COPY --from=planner /app/recipe.json recipe.json
ENV CARGO_NET_GIT_FETCH_WITH_CLI=true
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust:1.63.0-slim-buster as builder
WORKDIR app
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    --allow-unauthenticated zlib1g-dev \
    && apt-get clean && rm -rf /var/lib/apt/lists/*
COPY . .
COPY --from=cacher /app/target target
COPY --from=cacher $CARGO_HOME $CARGO_HOME
RUN cargo build --release --locked --no-default-features
RUN rm -f target/release/deps/git_cliff*

FROM debian:buster-slim as runner
WORKDIR app
COPY --from=builder /app/target/release/git-cliff /usr/local/bin
ENTRYPOINT ["git-cliff"]
