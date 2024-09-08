FROM rust:1.73.0-bookworm as builder

WORKDIR /usr/src/energy-reporting
COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/src/energy-reporting/target \
    cargo install --path .

FROM debian:bookworm-slim as app

WORKDIR /usr/local/bin
COPY --from=builder /usr/local/cargo/bin/energy-reporting /usr/local/bin/energy-reporting
CMD ["energy-reporting"]
