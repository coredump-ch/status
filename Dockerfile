# Build spaceapi
FROM rust:1.40 as builder
WORKDIR /source
COPY . /source
RUN cargo build --release && \
    cp target/release/coredump-status /usr/local/bin/coredump-status && \
    cd / && rm -rf /source

# Create runtime container
# Note that we need a small init process for PID 1 that forwards signals.
# See https://github.com/Yelp/dumb-init
FROM debian:stable-slim
RUN apt-get update && apt-get install dumb-init && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/bin/coredump-status /usr/local/bin/
ENV RUST_LOG=warn,spaceapi=info,spaceapi_server=info \
    REDIS_HOST=spaceapi-redis

# Entry point
EXPOSE 3000
ENTRYPOINT ["/usr/bin/dumb-init", "--"]
CMD ["/usr/local/bin/coredump-status", "-i", "0.0.0.0", "-p", "3000"]
