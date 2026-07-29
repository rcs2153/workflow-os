FROM rust:1.95-bookworm AS builder

WORKDIR /workspace
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates
RUN cargo build --locked --release -p workflow-hosted

FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install --yes --no-install-recommends ca-certificates curl \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /workspace/target/release/workflow-os-hosted /usr/local/bin/workflow-os-hosted

USER 65532:65532
ENTRYPOINT ["/usr/local/bin/workflow-os-hosted"]
