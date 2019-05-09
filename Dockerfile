# Build spaceapi
FROM rust:1.34 as builder
WORKDIR /source
COPY . /source
RUN cargo build --release && \
    cp target/release/coredump-status /usr/local/bin/coredump-status && \
    cd / && rm -rf /source

# Create runtime container
FROM debian:stable-slim
COPY --from=builder /usr/local/bin/coredump-status /usr/local/bin/
ENV RUST_LOG=warn,spaceapi=info,spaceapi_server=info \
    REDIS_HOST=spaceapi-redis

# Entry point
EXPOSE 3000
CMD ["/usr/local/bin/coredump-status", "-i", "0.0.0.0", "-p", "3000"]
