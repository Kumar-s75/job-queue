FROM rust:1.88-slim-bookworm AS build
WORKDIR /app
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release && cp /app/target/release/forgequeue /tmp/forgequeue
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=build /tmp/forgequeue /usr/local/bin/forgequeue
ENV BIND_ADDR=0.0.0.0:8080
EXPOSE 8080
USER 65532:65532
ENTRYPOINT ["forgequeue"]
