FROM lukemathwalker/cargo-chef as planner
WORKDIR app
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM lukemathwalker/cargo-chef as cacher
WORKDIR app
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust:1.52-slim-buster as builder
WORKDIR app
COPY . .
COPY --from=cacher /app/target target
COPY --from=cacher $CARGO_HOME $CARGO_HOME
RUN cargo build --release --locked
RUN rm -f target/release/deps/git_cliff*

FROM debian:buster-slim as runner
WORKDIR app
COPY --from=builder /app/target/release/git-cliff /usr/local/bin
USER nobody
ENTRYPOINT ["git-cliff"]
