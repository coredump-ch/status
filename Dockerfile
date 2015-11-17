FROM debian:jessie
MAINTAINER Danilo Bargen <mail@dbrgn.ch>

ENV RUST_VERSION=1.4.0

# Build base system
RUN apt-get update && \
  DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
    build-essential \
    ca-certificates \
    curl \
    libssl-dev && \
  curl -sO https://static.rust-lang.org/dist/rust-$RUST_VERSION-x86_64-unknown-linux-gnu.tar.gz && \
  tar -xzf rust-$RUST_VERSION-x86_64-unknown-linux-gnu.tar.gz && \
  ./rust-$RUST_VERSION-x86_64-unknown-linux-gnu/install.sh --without=rust-docs && \
  DEBIAN_FRONTEND=noninteractive apt-get remove --purge -y curl && \
  DEBIAN_FRONTEND=noninteractive apt-get autoremove -y && \
  rm -rf \
    rust-$RUST_VERSION-x86_64-unknown-linux-gnu \
    rust-$RUST_VERSION-x86_64-unknown-linux-gnu.tar.gz \
    /var/lib/apt/lists/* \
    /tmp/* \
    /var/tmp/* && \
  mkdir /source

# Build spaceapi
WORKDIR /source
COPY . /source
RUN cargo build --release && \
    cp target/release/coredump-status /usr/local/bin/coredump-status && \
    cd / && rm -rf /source

# Set runtime related environment variables
ENV RUST_LOG=warn,spaceapi=info,spaceapi_server=info \
    REDIS_HOST=spaceapi-redis

# Entry point
EXPOSE 3000
CMD ["/usr/local/bin/coredump-status", "-i", "0.0.0.0", "-p", "3000"]
